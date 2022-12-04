use crate::AddRemoveId;
use ahash::RandomState;
use couch_rs::CouchDocument;
use rkyv::Archive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub const FOLLOWERS: &str = "followers";

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
pub struct Followers {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub diff: HashMap<Uuid, AddRemoveId, RandomState>,
}
