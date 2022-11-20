use serde::{Deserialize, Serialize};

pub mod couchdb;
#[cfg(feature = "server")]
pub mod rabbitmq;
#[cfg(feature = "server")]
pub mod sql;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum AddRemoveId {
    Added(u64),
    Removed(u64),
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CountedDiff {
    pub count: u64,
    pub diff: Vec<AddRemoveId>,
}
