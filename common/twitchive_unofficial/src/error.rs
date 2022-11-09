use reqwest::{Error, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TwitterApiError {
    #[error("Hit the Twitter Ratelimit")]
    RateLimit,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("idk")]
    UnknownError,
    #[error("ProxyErr")]
    Proxy,
    #[error("HTTP Error {0}")]
    HttpError(u16),
    #[error("Error while scraping: {0}")]
    Scrape(String),
}

impl From<reqwest::Error> for TwitterApiError {
    fn from(e: Error) -> Self {
        match e.status() {
            Some(sc) => TwitterApiError::HttpError(sc.as_u16()),
            None => TwitterApiError::UnknownError,
        }
    }
}
