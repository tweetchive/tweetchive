use couch_rs::CouchDocument;
use rkyv::Archive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use twtscrape::tweet::TweetType;
use uuid::Uuid;

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
    pub quoting_id: HashMap<Uuid, u64>,
    pub reply_to: u64,
    pub poster: u64,
    pub data: HashMap<Uuid, TweetType>,
}
