use crate::custom_diff::CountedDiff;
use ahash::RandomState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct FollowersDiff {
    pub user: u64,
    pub base: (u64, Vec<u64>),
    pub diffs: HashMap<u64, CountedDiff, RandomState>,
}
