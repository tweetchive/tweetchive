use crate::config::Config;
use couch_rs::Client;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

mod api;
mod config;

pub struct AppState {
    pub config: RwLock<Config>,
    pub sql: DatabaseConnection,
    pub couch_client: Client,
    pub
}

#[tokio::main]
async fn main() {}
