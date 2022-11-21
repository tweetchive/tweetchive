use crate::config::{Account, Config, Proxy};
use crate::pools::{AnonymousScraperManager, TwitterScraperManager};
use color_eyre::Result;
use dashmap::DashMap;
use deadpool::managed::{Object, Pool};
use deadpool_lapin::{Manager, Pool as LapinPool};
use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::ConnectionProperties;
use nanorand::WyRand;
use opentelemetry::sdk::export::trace::stdout;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use flume::{Receiver, Sender};
use s3::Bucket;
use tikv_jemallocator::Jemalloc;
use tokio::sync::{Mutex, OnceCell, RwLock};
use tracing::{info, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tweetchive_core::rabbitmq::{ArchivedUser, REQUEST_QUEUE};
use twtscrape::scrape::Scraper;

static GLOBAL: Jemalloc = Jemalloc;

mod browser;
mod config;
mod error;
mod media;
mod pools;
mod routes;
mod tweet;
mod user;
mod export;

pub struct AppState {
    pub config: RwLock<Config>,
    pub rabbitmq: LapinPool,
    pub account_pool: Pool<TwitterScraperManager, Object<Scraper>>,
    pub anon_pool: Pool<AnonymousScraperManager, Object<Scraper>>,
    pub s3: Bucket,
    pub tweet_done_channel: TweetDone,
    pub user_done_channel: UserDone,
}

pub struct TweetDone {
    
}

pub struct UserDone {
    pub receiver: Arc<Receiver<ArchivedUser>>,
    pub sender: Arc<Sender<ArchivedUser>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // config
    let config = Config::load().await?;
    let proxies = Arc::new(
        config
            .proxys
            .iter()
            .map(|x| (x.ip.clone(), x.clone()))
            .collect::<DashMap<String, Proxy>>(),
    );
    // log into these accounts

    // instrumentation
    let tracer = stdout::new_pipeline().install_simple();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // rabbitmq
    let rmq_manager = Manager::new(&config.rabbitmq.address, ConnectionProperties::default());
    let rmq_pool = LapinPool::builder(manager)
        .max_size(config.rabbitmq.pool_size as usize)
        .build()?;

    // scraping pool
    let scrape_manager = TwitterScraperManager {
        accounts: Arc::new(RwLock::new(config.accounts.clone())),
        proxies: proxies.clone(),
        rng: Mutex::new(WyRand::new()),
    };
    let scrape_pool: Pool<TwitterScraperManager, Object<Scraper>> = Pool::builder(scrape_manager)
        .max_size(config.accounts.len())
        .build()?;

    let anon_manager = AnonymousScraperManager {
        proxies,
        rng: Mutex::new(WyRand::new()),
    };
    let anon_pool: Pool<AnonymousScraperManager, Object<Scraper>> = Pool::builder(anon_manager)
        .max_size(config.accounts.len())
        .build()?;

    let s3 =

    let state = Arc::new(AppState {
        config: RwLock::new(config),
        rabbitmq: rmq_pool,
        account_pool: scrape_pool,
        anon_pool,
        s3: ()
    });
}

#[instrument]
pub async fn init_rabbitmq(pool: LapinPool) -> Result<LapinPool> {
    let rmq_con = pool.get().await.map_err(|e| {
        eprintln!("could not get rmq con: {}", e);
        e
    })?;

    let channel = rmq_con.create_channel().await?;

    let task_queue = channel
        .queue_declare(
            REQUEST_QUEUE,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!("Got Queue {REQUEST_QUEUE}");
    let consumer = channel
        .basic_consume(
            REQUEST_QUEUE,
            "request_eater",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
}
