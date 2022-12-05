use crate::AppState;
use ahash::RandomState;
use color_eyre::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::couchdb::followers::Followers;
use tweetchive_core::couchdb::following::Following;
use tweetchive_core::AddRemoveId;
use uuid::Uuid;

#[instrument]
pub async fn following(
    state: Arc<AppState>,
    id: u64,
) -> Result<HashMap<Uuid, AddRemoveId, RandomState>> {
    let document = state
        .couches
        .following
        .get::<Following>(&id.to_string())
        .await?;
    Ok(document.diff)
}

#[instrument]
pub async fn followers(
    state: Arc<AppState>,
    id: u64,
) -> Result<HashMap<Uuid, AddRemoveId, RandomState>> {
    let document = state
        .couches
        .followers
        .get::<Followers>(&id.to_string())
        .await?;
    Ok(document.diff)
}
