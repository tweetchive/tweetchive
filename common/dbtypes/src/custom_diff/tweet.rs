use sea_orm::prelude::Uuid;
use std::collections::HashMap;

pub struct TweetDiff {
    pub id: u64,
    pub user: u64,
}
