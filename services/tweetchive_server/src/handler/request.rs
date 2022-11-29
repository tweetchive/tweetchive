use crate::AppState;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn archive_tweet(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
}

#[instrument]
pub async fn archive_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
}

#[instrument]
pub async fn archive_user_by_handle(
    Extension(state): Extension<Arc<AppState>>,
    Path(handle): Path<String>,
) -> impl IntoResponse {
}

#[instrument]
pub async fn delete_and_ban_archive_profile(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
}

#[instrument]
pub async fn restrict_archive_profile(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
}

#[instrument]
pub async fn report_archive_profile(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
}

#[instrument]
pub async fn report_archive_tweet(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
}
