use crate::config::{Config, Proxy};
use crate::pools::{AnonymousScraperManager, TwitterScraperManager};
use crate::tweet::archive_tweet;
use crate::user::archive_user;
use axum::handler::Handler;
use axum::{Router, Server};
use color_eyre::Result;
use dashmap::DashMap;
use deadpool::managed::{Object, Pool};
use deadpool_lapin::{Manager, Pool as LapinPool};
use flume::{Receiver, Sender};
use futures::StreamExt;
use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::ConnectionProperties;
use nanorand::WyRand;
use opentelemetry::sdk::export::trace::stdout;
use s3::creds::Credentials;
use s3::Bucket;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tikv_jemallocator::Jemalloc;
use tokio::sync::{Mutex, RwLock};
use tracing::log::warn;
use tracing::{info, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tweetchive_core::rabbitmq::{
    ArchivalRequest, ArchivalType, ArchivedTweets, ArchivedUser, REQUEST_QUEUE,
};
use twtscrape::scrape::Scraper;

static GLOBAL: Jemalloc = Jemalloc;

mod browser;
mod config;
mod error;
mod export;
mod health;
mod media;
mod pools;
mod routes;
mod tweet;
mod user;

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
    pub receiver: Arc<Receiver<ArchivedTweets>>,
    pub sender: Arc<Sender<ArchivedTweets>>,
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
    let rmq_pool = LapinPool::builder(rmq_manager)
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

    let s3 = Bucket::new(
        &config.s3.name,
        config.s3.region.parse().unwrap(),
        Credentials::new(None, None, None, None, None).unwrap(),
    )
    .unwrap();

    let (twt_sender, twt_receiver) = {
        let (a, b) = flume::unbounded();
        (Arc::new(a), Arc::new(b))
    };
    let (usr_sender, usr_receiver) = {
        let (a, b) = flume::unbounded();
        (Arc::new(a), Arc::new(b))
    };

    let state = Arc::new(AppState {
        config: RwLock::new(config),
        rabbitmq: rmq_pool,
        account_pool: scrape_pool,
        anon_pool,
        s3,
        tweet_done_channel: TweetDone {
            receiver: twt_receiver,
            sender: twt_sender,
        },
        user_done_channel: UserDone {
            receiver: usr_receiver,
            sender: usr_sender,
        },
    });

    let state_clone = state.clone();

    tokio::task::spawn(async {
        listen(state_clone).await.expect("wtf listen died???");
    });

    let app = Router::new().merge(health::router()).with_state(state);

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[instrument]
pub async fn listen(state: Arc<AppState>) -> Result<LapinPool> {
    let rmq_con = state.rabbitmq.get().await.map_err(|e| {
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
    let mut consumer = channel
        .basic_consume(
            REQUEST_QUEUE,
            "request_eater",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    loop {
        match consumer.next().await {
            Some(delivery) => match delivery {
                Ok(delivery) => {
                    let request = match rkyv::from_bytes::<ArchivalRequest>(&delivery.data) {
                        Ok(v) => v,
                        Err(why) => {
                            warn!(error = why, "error deserializing request!");
                            continue;
                        }
                    };

                    let state_clone = state.clone();
                    info!(archive = request.archival_id, "Starting...");
                    tokio::task::spawn(async move {
                        let rslt = match request.arc_type {
                            ArchivalType::User { user } => {
                                archive_user(state_clone, request.archival_id, user).await
                            }
                            ArchivalType::TweetThread { tweet_id } => {
                                archive_tweet(state_clone, request.archival_id, tweet_id).await
                            }
                        };

                        if let Err(why) = rslt {
                            warn!(error = why, archive = request.archival_id, "Task Failed");
                            return;
                        }

                        info!(archive = request.archival_id, "Task Successful");
                    })
                }
                Err(why) => {
                    warn!(error = why, "bad delivery!");
                    continue;
                }
            },
            None => {
                warn!("no delivery!");
                continue;
            }
        }
    }
}
