use crate::error::TwitterApiError;
use chrono::{DateTime, NaiveDateTime, Utc};
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
static PROFILE_STATS_SELECTOR: Selector = Selector::parse("profile-stat-num").unwrap();

static PROFILE_LOCKED_SELECTOR: Selector = Selector::parse("icon-lock").unwrap();
static PROFILE_VERIFIED_SELECTOR: Selector = Selector::parse("icon-ok verified-icon").unwrap();

static TIMELINE_NEXT_SELECTOR: Selector = Selector::parse("show-more").unwrap();
static TIMELINE_TWEETS_SELECTOR: Selector = Selector::parse("timeline-item").unwrap();

static TWEET_SHOW_THREAD_SELECTOR: Selector = Selector::parse("show-thread").unwrap();
static TWEET_STATS_SELECTOR: Selector = Selector::parse("tweet-stats").unwrap();
static TWEET_CONTENT_SELECTOR: Selector = Selector::parse("tweet-content").unwrap();
static THREAD_SHOW_SELECTOR: Selector = Selector::parse("show-thread").unwrap();

const JOINDATE_STR: &str = "%R %p - %e %b %Y";

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

    pub async fn user_matters(&self, username: String) -> Result<PartialUser, TwitterApiError> {
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
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?;

        let username = parsed
            .select(&PROFILE_USERNAME_SELECTOR)
            .next()
            .map(|x| x.inner_html())
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?;

        let banner = parsed
            .select(&PROFILE_BANNER_SELECTOR)
            .next()
            .map(|x| x.first_child())
            .flatten()
            .map(|x| x.value().as_element())
            .flatten()
            .map(|x| x.attr("href").map(ToString::to_string))
            .flatten()
            .map(|url| urlencoding::decode(&url.replacen("/pic/", "", 1)).map(|x| x.to_string()))
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?
            .map_err(|why| TwitterApiError::Scrape(why.to_string()))?
            .to_string();

        let profile_picture = parsed
            .select(&PROFILE_AVATAR_SELECTOR)
            .next()
            .map(|x| x.value().attr("href").map(ToString::to_string))
            .flatten()
            .map(|url| urlencoding::decode(&url.replacen("/pic/", "", 1)).map(|x| x.to_string()))
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?
            .map_err(|why| TwitterApiError::Scrape(why.to_string()))?
            .to_string();

        let bio = parsed
            .select(&PROFILE_LOCATION_SELECTOR)
            .next()
            .map(|x| x.children().next().map(|elem| elem.value().as_text()))
            .flatten()
            .flatten()
            .map(|text| text.text.to_string())
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?;

        let location = parsed
            .select(&PROFILE_LOCATION_SELECTOR)
            .next()
            .map(|x| x.children().nth(1))
            .flatten()
            .map(|x| x.value().as_text())
            .flatten()
            .map(|x| x.to_string())
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?;

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
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?;

        let joindate = parsed
            .select(&PROFILE_JOINDATE_SELECTOR)
            .next()
            .map(|x| x.value().attr("title"))
            .flatten()
            .map(|datestr| DateTime::parse_from_str(datestr, JOINDATE_STR))
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?
            .map_err(|why| TwitterApiError::Scrape(why.to_string()))?;

        let tweets = parsed
            .select(&PROFILE_POSTS_SELECTOR)
            .next()
            .map(|x| x.select(&PROFILE_STATS_SELECTOR).next())
            .flatten()
            .map(|x| x.inner_html())
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?
            .parse::<u64>()
            .map_err(|why| TwitterApiError::Scrape(why.to_string()))?;

        let followers = parsed
            .select(&PROFILE_FOLLOWERS_SELECTOR)
            .next()
            .map(|x| x.select(&PROFILE_STATS_SELECTOR).next())
            .flatten()
            .map(|x| x.inner_html())
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?
            .parse::<u64>()
            .map_err(|why| TwitterApiError::Scrape(why.to_string()))?;

        let follows = parsed
            .select(&PROFILE_FOLLOWING_SELECTOR)
            .next()
            .map(|x| x.select(&PROFILE_STATS_SELECTOR).next())
            .flatten()
            .map(|x| x.inner_html())
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?
            .parse::<u64>()
            .map_err(|why| TwitterApiError::Scrape(why.to_string()))?;

        let likes = parsed
            .select(&PROFILE_LIKES_SELECTOR)
            .next()
            .map(|x| x.select(&PROFILE_STATS_SELECTOR).next())
            .flatten()
            .map(|x| x.inner_html())
            .ok_or(TwitterApiError::Scrape("Element is null".to_string()))?
            .parse::<u64>()
            .map_err(|why| TwitterApiError::Scrape(why.to_string()))?;

        let private = parsed.select(&PROFILE_LIKES_SELECTOR).next().is_some();
        let verified = parsed.select(&PROFILE_LIKES_SELECTOR).next().is_some();

        Ok(PartialUser {
            name,
            username,
            bio,
            location,
            link: website,
            joindate: DateTime::<Utc>::from(joindate),
            tweets,
            profile_picture,
            following: follows,
            followers,
            likes,
            private,
            banner_url: banner,
            verified,
        })
    }

    pub async fn user_timeline(
        &self,
        username: String,
        pagination: Option<String>,
    ) -> Result<(Option<String>, Vec<TimelineTweet>), TwitterApiError> {
        let page = match pagination {
            Some(cursor) => {
                self.requester
                    .get(format!(
                        "{}/{username}/with_replies/?cursor={cursor}",
                        self.config.nitter_address
                    ))
                    .send()
                    .await?
                    .text()
                    .await?
            }
            None => {
                self.requester
                    .get(format!(
                        "{}/{username}/with_replies",
                        self.config.nitter_address
                    ))
                    .send()
                    .await?
                    .text()
                    .await?
            }
        };

        let parsed = Html::parse_document(&page);

        Err()
    }
}

pub struct PartialUser {
    pub name: String,
    pub username: String,
    pub bio: String,
    pub location: String,
    pub link: String,
    pub joindate: DateTime<Utc>,
    pub tweets: u64,
    pub profile_picture: String,
    pub following: u64,
    pub followers: u64,
    pub likes: u64,
    pub private: bool,
    pub banner_url: String,
    pub verified: bool,
}

pub struct TimelineTweet {
    pub user: String,
    pub id: u64,
    pub is_retweet: bool,
    pub is_thread: bool,
    pub quote_retweet: Option<u64>,
    pub reply_to: Option<Vec<u64>>,
    pub date: DateTime<Utc>,
    pub likes: u64,
    pub retweets: u64,
    pub qrts: u64,
    pub replies: u64,
    pub attachments: bool,
}
