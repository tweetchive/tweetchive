use crate::AddRemoveId;
use ahash::HashSet;
use couch_rs::CouchDocument;
use rkyv::Archive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub const FOLLOWING: &str = "following";

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
pub struct Following {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub base_state: HashSet<u64>,
    pub base_snapshot: Uuid,
    pub diff: HashMap<Uuid, AddRemoveId>,
    pub current_set: HashSet<u64>,
    pub current_snapshot: Uuid,
}
