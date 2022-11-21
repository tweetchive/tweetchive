use chrono::{DateTime, Utc};
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use twtscrape::tweet::{Media, Tweet};
use twtscrape::user::User;
use uuid::Uuid;

// Initalization

pub const REGISTRATION_QUEUE: &str = "register_me";
pub const REGISTRATION_QUEUE_CALLBACK: &str = "register_me";

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct RegisterMePlease {
    pub name: String,
    pub weight: u32,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub enum RegistrationResult {
    Ok { machine_id: u8 },
    Err(String),
}

// Request

pub const REQUEST_QUEUE: &str = "request";

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct ArchivalRequest {
    pub archival_id: Uuid,
    pub arc_type: ArchivalType,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub enum ArchivalType {
    User { user: String },
    TweetThread { tweet_id: u64 },
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct ArchiveUserPrevious {
    pub tweet: u64,
}

// Response

pub const USER_ARCHIVED_QUEUE: &str = "user_archived";
pub const TWEETS_ARCHIVED_QUEUE: &str = "tweets_archived";
pub const MEDIA_ARCHIVED_QUEUE: &str = "media_archived";

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct ArchivedUser {
    pub archival_id: Uuid,
    pub user: Result<ArchivedUserData, String>,
    pub retrieved: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct ArchivedUserData {
    pub user: User,
    pub others: Vec<User>,
    pub tweets: Vec<Tweet>,
    pub media: Vec<ArchivedMedia>,
    pub followers: Vec<User>,
    pub following: Vec<User>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct ArchivedTweets {
    pub archival_id: Uuid,
    pub tweets: Result<ArchivedTweetData, String>,
    pub retrieved: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct ArchivedTweetData {
    pub tweets: Vec<Tweet>,
    pub users: Vec<User>,
    pub media: Vec<ArchivedMedia>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub struct ArchivedMedia {
    pub archival_id: Uuid,
    pub media_id: String,
    pub media_type: MediaType,
    pub content_type: String,
    pub retrieved: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, Archive)]
pub enum MediaType {
    Picture,
    Gif,
    Video,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MediaType::Picture => {
                    "Picture"
                }
                MediaType::Gif => {
                    "Gif"
                }
                MediaType::Video => {
                    "Video"
                }
            }
        )
    }
}

// control
