use crate::AppState;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn status(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if let Ok(p) = params.get("ssk") {
        if p == state.config.read().await.ssk {
            Ok(())
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn router() -> Router {
    Router::new().route("/status", get(status))
}
