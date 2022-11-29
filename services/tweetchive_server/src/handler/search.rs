use crate::AppState;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn status(
    Extension(state): Extension<Arc<AppState>>,
    Path(query): Path<String>,
) -> impl IntoResponse {
}
