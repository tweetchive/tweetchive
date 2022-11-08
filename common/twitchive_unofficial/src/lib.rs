use crate::error::TwitterApiError;
use chrono::{DateTime, Utc};
use reqwest::header::ToStrError;
use reqwest::{header, Client, ClientBuilder};
use scraper::{Html, Selector};
use std::ops::Deref;
use tokio_rayon::spawn;

mod error;

static PROFILE_FULLNAME_SELECTOR: Selector = Selector::parse("profile-card-fullname").unwrap();
static PROFILE_USERNAME_SELECTOR: Selector = Selector::parse("profile-card-username").unwrap();
static PROFILE_BANNER_SELECTOR: Selector = Selector::parse("profile-banner").unwrap();
static PROFILE_AVATAR_SELECTOR: Selector = Selector::parse("profile-card-avatar").unwrap();
static PROFILE_BIO_SELECTOR: Selector = Selector::parse("profile-bio").unwrap();
static PROFILE_LOCATION_SELECTOR: Selector = Selector::parse("profile-location").unwrap();
static PROFILE_WEBSITE_SELECTOR: Selector = Selector::parse("profile-website").unwrap();
static PROFILE_JOINDATE_SELECTOR: Selector = Selector::parse("profile-joindate").unwrap();
static PROFILE_POSTS_SELECTOR: Selector = Selector::parse("posts").unwrap();
static PROFILE_FOLLOWING_SELECTOR: Selector = Selector::parse("following").unwrap();
static PROFILE_FOLLOWERS_SELECTOR: Selector = Selector::parse("followers").unwrap();
static PROFILE_LIKES_SELECTOR: Selector = Selector::parse("likes").unwrap();

pub struct Config {
    pub nitter_address: String,
}

pub struct Scraper {
    config: Config,
    requester: Client,
}

impl Scraper {
    pub fn new(config: Config) -> Result<Self, TwitterApiError> {
        let clientmaker = ClientBuilder::new().use_rustls_tls().cookie_store(true);

        let requester = clientmaker
            .build()
            .map_err(|_| TwitterApiError::UnknownError)?;

        Ok(Self { config, requester })
    }

    pub async fn user_matters(&self, username: String) -> Result<(), TwitterApiError> {
        let page = self
            .requester
            .get(format!("{}/{username}", self.config.nitter_address))
            .send()
            .await?
            .text()
            .await?;

        let parsed = Html::parse_document(&page);

        let name = parsed
            .select(&PROFILE_FULLNAME_SELECTOR)
            .next()
            .map(|x| x.inner_html())
            .ok_or(|_| TwitterApiError::Scrape)?;

        let username = parsed
            .select(&PROFILE_USERNAME_SELECTOR)
            .next()
            .map(|x| x.inner_html())
            .ok_or(|_| TwitterApiError::Scrape)?;

        let banner = parsed
            .select(&PROFILE_BANNER_SELECTOR)
            .nth(1)
            .map(|x| x.value().attr("href").map(ToString::to_string))
            .flatten()
            .map(|url| urlencoding::decode(&url.replacen("/pic/", "", 1)))
            .ok_or(|_| TwitterApiError::Scrape)?
            .map_err(|_| TwitterApiError::Scrape)?
            .to_string();

        let bio = parsed
            .select(&PROFILE_LOCATION_SELECTOR)
            .next()
            .map(|x| x.children().next().map(|elem| elem.value().as_text()))
            .flatten()
            .flatten()
            .map(|text| text.text.to_string())
            .ok_or(|_| TwitterApiError::Scrape)?;

        let location = parsed
            .select(&PROFILE_LOCATION_SELECTOR)
            .next()
            .map(|x| x.children().nth(1))
            .flatten()
            .map(|x| x.value().as_text())
            .flatten()
            .map(|x| x.to_string())
            .ok_or(|_| TwitterApiError::Scrape)?;

        let website = parsed
            .select(&PROFILE_WEBSITE_SELECTOR)
            .next()
            .map(|x| x.children().next().map(|c| c.last_child()))
            .flatten()
            .flatten()
            .map(|x| {
                x.value()
                    .as_element()
                    .map(|elem| elem.attr("href").map(ToString::to_string))
            })
            .flatten()
            .flatten()
            .ok_or(|_| TwitterApiError::Scrape)?;

        let username = parsed
            .select(&PROFILE_USERNAME_SELECTOR)
            .next()
            .map(|x| x.inner_html())
            .ok_or(|_| TwitterApiError::Scrape)?;

        let username = parsed
            .select(&PROFILE_USERNAME_SELECTOR)
            .next()
            .map(|x| x.inner_html())
            .ok_or(|_| TwitterApiError::Scrape)?;

        let username = parsed
            .select(&PROFILE_USERNAME_SELECTOR)
            .next()
            .map(|x| x.inner_html())
            .ok_or(|_| TwitterApiError::Scrape)?;

        Ok(())
    }

    pub async fn user_next_timeline(&self, pagination: String) -> Result<(), TwitterApiError> {}
}

pub struct User {
    pub name: String,
    pub username: String,
    pub bio: String,
    pub location: String,
    pub link: String,
    pub joindate: DateTime<Utc>,
    pub birthday: DateTime<Utc>,
    pub tweets: u64,
    pub profile_picture: String,
    pub following: u64,
    pub followers: u64,
    pub likes: u64,
    pub suspended: bool,
    pub private: bool,
    pub banner_url: String,
}
