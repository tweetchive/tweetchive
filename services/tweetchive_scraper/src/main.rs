use crate::config::{Account, Config, Proxy};
use crate::pools::TwitterScraperManager;
use color_eyre::Result;
use deadpool::managed::{Object, Pool};
use deadpool_lapin::{Manager, Pool as LapinPool};
use lapin::ConnectionProperties;
use nanorand::WyRand;
use opentelemetry::sdk::export::trace::stdout;
use std::sync::Arc;
use tikv_jemallocator::Jemalloc;
use tokio::sync::{Mutex, OnceCell, RwLock};
use tracing::instrument;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use twtscrape::scrape::Scraper;

static GLOBAL: Jemalloc = Jemalloc;

mod config;
mod error;
mod pools;
mod routes;
mod user;
mod browser;

pub struct AppState {
    pub config: RwLock<Config>,
    pub rabbitmq: LapinPool,
    pub account_pool: Pool<TwitterScraperManager, Object<Scraper>>
}

pub static STATE: Arc<OnceCell<AppState>>  = Arc::new(OnceCell::new());

#[tokio::main]
async fn main() -> Result<()> {
    // config
    let config = Arc::new(Config::load().await?);

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
        rng: Mutex::new(WyRand::new()),
    };
    let scrape_pool: Pool<TwitterScraperManager, Object<Scraper>> = Pool::builder(scrape_manager)
        .max_size(config.accounts.len())
        .build()?;
}

#[instrument]
pub async fn init_rabbitmq(pool: LapinPool) -> Result<LapinPool> {
    let rmq_con = pool.get().await.map_err(|e| {
        eprintln!("could not get rmq con: {}", e);
        e
    })?;

    let channel = rmq_con.create_channel().await?;

    let task_queue = channel.queue_declare(
        ""
    )
}
