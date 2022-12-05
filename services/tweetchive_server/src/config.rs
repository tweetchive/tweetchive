use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::env::var;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub database: String,
    pub super_secret_key: String,
    pub workers: Vec<Worker>,
    pub github_client_id: String,
    pub github_client_secret: String,
}

impl Config {
    pub async fn load() -> Result<Config> {
        let mut cfgstr = String::new();
        File::open(var("TWTCHIVE_CONFIG_PATH").unwrap_or("twtchive.toml".to_string()))
            .await?
            .read_to_string(&mut cfgstr)
            .await?;
        Ok(toml::from_str::<Self>(&cfgstr)?)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Worker {
    pub ip: String,
    pub ssk: String,
}
