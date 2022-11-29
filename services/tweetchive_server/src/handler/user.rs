use crate::AppState;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;

pub fn profile(
    Extension(state): Extension<Arc<AppState>>,
    Path(handle): Path<String>,
) -> impl IntoResponse {
}
