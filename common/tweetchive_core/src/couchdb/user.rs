use couch_rs::CouchDocument;
use rkyv::Archive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use twtscrape::user::User;
use uuid::Uuid;

pub const USERS: &str = "users";

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
pub struct UserArchive {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub data: HashMap<Uuid, User>,
    pub media: HashMap<Uuid, UserMedia>,
}

#[derive(
    Copy,
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
pub struct UserMedia {
    pub profile_picture: Uuid,
    pub banner: Uuid,
    pub is_nft: bool,
}
