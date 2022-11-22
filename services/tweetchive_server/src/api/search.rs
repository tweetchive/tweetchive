use crate::AppState;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use axum::http::StatusCode;
use tracing::instrument;
use tweetchive_core::sql;

#[derive(Serialize, Deserialize)]
pub struct SearchByHandleReply {
    pub id: u64,
    pub handle: String,
}

#[instrument]
pub async fn search(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if let Some(handle) = params.get("handle") {
        let req: impl Iterator<Item = SearchByHandleReply> = sql::user::Entity::find()
            .filter(sql::user::Column::Handle.contains(handle))
            .all(&state.sql)
            .await?
            .into_iter()
            .map(|x| SearchByHandleReply {
                id: x.id,
                handle: x.handle,
            });

        return Ok(Json(req));
    }
    if let Some(user_id) = params.get("user_id") {
        let id = user_id.parse::<u64>()?;
        let req = sql::user::Entity::find_by_id(id).one(&state.sql).await?;

        return Ok(Json(req));
    }

    Err(StatusCode::BAD_REQUEST)
}
