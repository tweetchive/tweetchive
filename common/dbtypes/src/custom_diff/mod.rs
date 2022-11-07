use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod followers;
mod following;
mod likes;
mod quote_retweets;
mod retweets;
mod tweet;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum AddRemoveId {
    Added(u64),
    Removed(u64),
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CountedDiff {
    pub count: u64,
    pub diff: Vec<AddRemoveId>,
}
