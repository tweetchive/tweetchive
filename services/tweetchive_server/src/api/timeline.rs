use crate::api::SnapshotTag;
use crate::AppState;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Extension;
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

pub fn timeline_api_req_url(user_id: u64) -> String {
    format!("api/timeline/{user_id}")
}

#[derive(Deserialize)]
struct Pagination {
    page: u32,
}

#[instrument]
pub async fn user(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
    page: Option<Query<Pagination>>,
) -> impl IntoResponse {
}
