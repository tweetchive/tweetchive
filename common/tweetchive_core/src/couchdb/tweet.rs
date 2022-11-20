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
)]
#[cfg_attr(feature = "server", derive(couch_rs::CouchDocument))]
pub struct Tweet {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub conversation_id: u64,
    pub data: HashMap<Uuid, TweetType>,
}
