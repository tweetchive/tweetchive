use crate::setup::tweet::{
    LatestRepliesToTweetOutput, LatestTweetsOfConversationOutput, CONVERSATION_LATEST,
    TWEET_BY_ALSO_IDS, TWEET_LATEST,
};
use crate::AppState;
use color_eyre::Result;
use couch_rs::types::query::QueryParams;
use couch_rs::types::view::{RawViewCollection, ViewItem};
use serde_json::Value;
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::couchdb::tweet::{Tweet, TWEETS};

#[instrument]
pub async fn timeline(
    state: Arc<AppState>,
    id: u64,
    page: Option<u32>,
) -> Result<Option<Vec<LatestRepliesToTweetOutput>>> {
    let mut query = match page {
        Some(p) => QueryParams::default().skip(30 * p as u64),
        None => QueryParams::default(),
    };

    query = query.key(id);
    query = query.limit(30);

    let mut data: RawViewCollection<u64, LatestRepliesToTweetOutput> = state
        .couches
        .tweets
        .query(TWEETS, TWEET_LATEST, Some(query))
        .await?;

    Ok(Some(data.rows.into_iter().map(|row| row.value).collect()))
}

#[instrument]
pub async fn thread(
    state: Arc<AppState>,
    conversation_id: u64,
) -> Result<Option<Vec<LatestTweetsOfConversationOutput>>> {
    let query = QueryParams::default().key(conversation_id);

    let mut data: RawViewCollection<u64, LatestTweetsOfConversationOutput> = state
        .couches
        .tweets
        .query(TWEETS, CONVERSATION_LATEST, Some(query))
        .await?;

    Ok(Some(data.rows.into_iter().map(|row| row.value).collect()))
}

#[instrument]
pub async fn tweet(state: Arc<AppState>, id: u64) -> Result<Option<Tweet>> {
    let query = QueryParams::default().key(id);

    let mut id: RawViewCollection<u64, u64> = state
        .couches
        .tweets
        .query(TWEETS, TWEET_BY_ALSO_IDS, Some(query))
        .await?;

    let actual_id = match id.rows.first() {
        None => return Ok(None),
        Some(aid) => match &aid.id {
            None => return Ok(None),
            Some(id) => id,
        },
    };

    Ok(Some(state.couches.tweets.get(actual_id).await?))
}
