use crate::herr::HResult;
use crate::setup::tweet::{
    LatestRepliesToTweetOutput, LatestTweetsOfConversationOutput, CONVERSATION_LATEST, TWEET_LATEST,
};
use crate::AppState;
use couch_rs::types::query::QueryParams;
use couch_rs::types::view::RawViewCollection;
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::couchdb::tweet::{Tweet, TWEETS};

#[instrument]
pub async fn timeline(
    state: Arc<AppState>,
    id: u64,
    page: Option<u32>,
) -> HResult<Vec<LatestRepliesToTweetOutput>> {
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

    Ok(data.rows.into_iter().map(|row| row.value).collect())
}

#[instrument]
pub async fn thread(
    state: Arc<AppState>,
    conversation_id: u64,
) -> HResult<Vec<LatestTweetsOfConversationOutput>> {
    let query = QueryParams::default().key(conversation_id);

    let mut data: RawViewCollection<u64, LatestTweetsOfConversationOutput> = state
        .couches
        .tweets
        .query(TWEETS, CONVERSATION_LATEST, Some(query))
        .await?;

    Ok(data.rows.into_iter().map(|row| row.value).collect())
}

#[instrument]
pub async fn tweet(state: Arc<AppState>, id: u64) -> HResult<Tweet> {
    Ok(state.couches.tweets.get(&id.to_string()).await?)
}
