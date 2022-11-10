use crate::error::TwitterApiError;
use chrono::{Date, DateTime, NaiveDateTime, Utc};
use reqwest::header::ToStrError;
use reqwest::{header, Client, ClientBuilder};
use scraper::node::Text;
use scraper::{ElementRef, Html, Selector};
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
static TWEET_LINK_SELECTOR: Selector = Selector::parse("tweet-link").unwrap();
static TWEET_STATS_SELECTOR: Selector = Selector::parse("tweet-stats").unwrap();
static TWEET_CONTENT_SELECTOR: Selector = Selector::parse("tweet-content media-body").unwrap();
static TWEET_PINNED_SELECTOR: Selector = Selector::parse("pinned").unwrap();
static TWEET_UNAVAILABLE_SELECTOR: Selector = Selector::parse("unavailable-box").unwrap();
static TWEET_RETAIL_SELECTOR: Selector = Selector::parse("retweet-header").unwrap();
static TWEET_ATTACHMENT_SELECTOR: Selector = Selector::parse("attachments").unwrap();
static TWEET_DATE_SELECTOR: Selector = Selector::parse("tweet-date").unwrap();
static THREAD_SHOW_SELECTOR: Selector = Selector::parse("show-thread").unwrap();
static THREAD_MAIN_SELECTOR: Selector = Selector::parse("#m").unwrap();
static THREAD_REPLYING_TO: Selector = Selector::parse("replying-to").unwrap();

const JOINDATE_STR: &str = "%R %p - %e %b %Y";
const TWEETDATE_STR: &str = "%b %e, %Y · %R%p UTC";
//Nov 9, 2022 · 3:58 PM UTC
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

        parsed
            .select(&TIMELINE_TWEETS_SELECTOR)
            .map(|timeline_item| {});

        Err()
    }

    pub fn parse_tweet_reply(html: &Html, tweetlink: String) -> Option<(Tweet, bool)> {


        let (mut main_tweet, attachments) = Self::parse_tweet_nonreply(tweet)?;

        // find "replying to"
        let
    }

    pub fn parse_tweet_nonreply(tweet: ElementRef<'_>) -> Option<(Tweet, bool)> {
        let tweet_link = tweet
            .select(&TWEET_LINK_SELECTOR)
            .next()
            .map(|x| {
                x.value().attr("href").map(|x| {
                    x.split('/')
                        .last()
                        .map(|x| x.split("#m").next().map(str::parse::<u64>).map(Result::ok))
                })
            })
            .flatten()
            .flatten()
            .flatten()
            .flatten()?;

        let pinned = tweet.select(&TWEET_PINNED_SELECTOR).next().is_some();

        let tweet_content = tweet
            .select(&TWEET_CONTENT_SELECTOR)
            .next()
            .map(|x| x.inner_html())
            .unwrap_or(String::new());
        let replies = tweet
            .select(&TWEET_STATS_SELECTOR)
            .next()
            .map(|x| {
                x.first_child().map(|x| {
                    x.value()
                        .as_text()
                        .map(|x| x.deref().to_string().parse().ok())
                })
            })
            .flatten()
            .flatten()
            .flatten()
            .unwrap_or(0);
        let retweets = tweet
            .select(&TWEET_STATS_SELECTOR)
            .next()
            .map(|x| {
                x.first_child().map(|x| {
                    x.value()
                        .as_text()
                        .map(|x| x.deref().to_string().parse().ok())
                })
            })
            .flatten()
            .flatten()
            .flatten()
            .unwrap_or(0);
        let quotes = tweet
            .select(&TWEET_STATS_SELECTOR)
            .next()
            .map(|x| {
                x.first_child().map(|x| {
                    x.value()
                        .as_text()
                        .map(|x| x.deref().to_string().parse().ok())
                })
            })
            .flatten()
            .flatten()
            .flatten()
            .unwrap_or(0);
        let likes = tweet
            .select(&TWEET_STATS_SELECTOR)
            .next()
            .map(|x| {
                x.first_child().map(|x| {
                    x.value()
                        .as_text()
                        .map(|x| x.deref().to_string().parse().ok())
                })
            })
            .flatten()
            .flatten()
            .flatten()
            .unwrap_or(0);

        let tweet_date = tweet
            .select(&TWEET_DATE_SELECTOR)
            .next()
            .map(|x| x.first_child().map(|x| x.value().as_element()))
            .flatten()
            .flatten()
            .map(|x| x.attr("title"))
            .flatten()
            .map(|datestr| NaiveDateTime::parse_from_str(datestr, TWEETDATE_STR).ok())
            .flatten()
            .map(|naive| DateTime::<Utc>::from_local(naive, Utc))
            .unwrap_or(Utc::now());

        let tweet_has_attachments = tweet.select(&TWEET_ATTACHMENT_SELECTOR).next().is_some();

        Some((
            Tweet {
                id: tweet_link,
                content: tweet_content,
                reply_tweet_id: None,
                reply_users: None,
                replies,
                quotes,
                retweets,
                likes,
                posted: tweet_date,
                has_attachments: tweet_has_attachments,
            },
            pinned,
        ))
    }
}

pub struct Timeline {
    pub pinned_tweet: Option<Tweet>,
    pub tweets: Vec<Tweet>,
}

pub struct Tweet {
    pub id: u64,
    pub content: String,
    pub reply_tweet_id: Option<u64>,
    pub reply_users: Option<Vec<String>>,

    pub replies: u64,
    pub quotes: u64,
    pub retweets: u64,
    pub likes: u64,

    pub posted: DateTime<Utc>,
    pub has_attachments: bool,
}
