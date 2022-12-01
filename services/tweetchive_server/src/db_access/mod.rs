use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod follows;
pub mod snapshot;
pub mod tweet;
pub mod user;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SnapshotTag {
    Uuid(Uuid),
    String(String),
}
