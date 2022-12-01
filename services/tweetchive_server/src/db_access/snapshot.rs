use crate::AppState;
use chrono::{DateTime, Utc};
use color_eyre::Result;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::sql;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum SnapshotStatus {
    Ongoing,
    Ended(DateTime<Utc>),
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotSearchResponse {
    pub snapshot: Uuid,
    pub init: DateTime<Utc>,
    pub args: String,
    pub status: SnapshotStatus,
    pub started_by: String,
    pub priority: i32,
}

#[instrument]
pub async fn snapshot(
    state: Arc<AppState>,
    snapshot: Uuid,
) -> Result<Option<SnapshotSearchResponse>> {
    Ok(sql::snapshots::Entity::find_by_id(snapshot)
        .one(&state.sql)
        .await?
        .map(|x| {
            let status = match x.finish {
                Some(s) => SnapshotStatus::Ended(s),
                None => SnapshotStatus::Ongoing,
            };

            SnapshotSearchResponse {
                snapshot,
                init: x.start,
                args: x.init_args,
                status,
                started_by: x.started_by,
                priority: x.priority,
            }
        }))
}
