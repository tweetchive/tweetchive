use couch_rs::CouchDocument;
use rkyv::Archive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use twtscrape::tweet::TweetType;
use uuid::Uuid;

pub const TWEETS: &str = "tweets";

#[derive(
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    CouchDocument,
)]
pub struct Tweet {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    pub also_ids: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub conversation_id: u64,
    pub reply_to: u64,
    pub author: u64,
    pub data: HashMap<Uuid, TweetType>,
    pub tweet_media: HashMap<Uuid, Vec<TweetMedia>>,
}

#[derive(
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub struct TweetMedia {
    pub file: Uuid,
    pub file_type: String,
    pub key: String,
    pub alt_text: String,
    pub views: Option<u32>,
}
