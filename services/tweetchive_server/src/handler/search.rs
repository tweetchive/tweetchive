use crate::AppState;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Extension, Form};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub search: String,
}

#[instrument]
pub async fn status(
    Extension(state): Extension<Arc<AppState>>,
    Form(search): Form<SearchQuery>,
) -> impl IntoResponse {
}
