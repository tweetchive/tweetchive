use crate::AppState;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn userdata(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {

    match state.couch.

}
