use crate::api::SnapshotTag;
use crate::AppState;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Extension;
use couch_rs::types::query::QueryParams;
use couch_rs::types::view::CouchFunc;
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

pub fn timeline_api_req_url(user_id: u64) -> String {
    format!("api/timeline/{user_id}")
}

pub fn timeline_of_user(user_id: u64) -> CouchFunc {
    CouchFunc {
        map: format!("function (doc) {{ if (doc.poster == {user_id}) {{ emit(doc) }} }} "),
        reduce: None,
    }
}

#[derive(Deserialize)]
struct Pagination {
    cursor: String,
}

#[instrument]
pub async fn timeline(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
    page: Option<Query<Pagination>>,
) -> impl IntoResponse {
    let query = match page {
        Some(p) => QueryParams::default().start_key_doc_id(&p.cursor),
        None => QueryParams::default(),
    };

    state.couches.tweets.query(Some(query))
}
