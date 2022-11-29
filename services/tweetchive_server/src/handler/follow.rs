use crate::AppState;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn following(
    Extension(state): Extension<Arc<AppState>>,
    Path(handle): Path<String>,
) -> impl IntoResponse {
}

#[instrument]
pub async fn followers(
    Extension(state): Extension<Arc<AppState>>,
    Path(handle): Path<String>,
) -> impl IntoResponse {
}
