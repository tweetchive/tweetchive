use crate::AppState;
use ahash::{HashMap, RandomState};
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::AddRemoveId;
use uuid::Uuid;

#[instrument]
pub async fn following(
    state: Arc<AppState>,
    id: u64,
) -> Result<HashMap<Uuid, AddRemoveId, RandomState>> {
}
