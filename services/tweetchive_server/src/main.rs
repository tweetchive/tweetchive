use crate::config::Config;
use couch_rs::database::Database;
use couch_rs::Client;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

mod api;
mod config;
mod setup;

pub struct AppState {
    pub config: RwLock<Config>,
    pub sql: DatabaseConnection,
    pub couch_client: Client,
    pub couches: Couches,
}

pub struct Couches {
    pub user: Database,
    pub tweets: Database,
    pub followers: Database,
    pub following: Database,
}

#[tokio::main]
async fn main() {}
