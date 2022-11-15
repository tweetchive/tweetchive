use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::env::var;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub machine_name: String,
    pub ssk: String,
    pub worker: Worker,
    pub accounts: Vec<Account>,
}

impl Config {
    pub async fn load() -> Result<Config> {
        let mut cfgstr = String::new();
        File::open(var("SCRAPER_CONFIG_PATH").unwrap_or("scraper.toml".to_string()))
            .await?
            .read_to_string(&mut cfgstr)
            .await?;
        Ok(toml::from_str::<Self>(&cfgstr)?)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Worker {
    pub search_workers: u16,
    pub tweet_workers: u16,
    pub profile_workers: u16,
    pub download_workers: u16,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub assigned_proxy: Option<Proxy>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Proxy {
    pub ip: String,
    pub username: String,
    pub password: String,
}
