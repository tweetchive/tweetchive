use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod search;
mod tweet;
mod user;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SnapshotTag {
    Uuid(Uuid),
    String(String),
}
