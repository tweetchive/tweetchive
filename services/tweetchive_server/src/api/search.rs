use crate::AppState;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use color_eyre::Report;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use couch_rs::types::find::FindQuery;
use tracing::instrument;
use tweetchive_core::sql;
use url::Url;

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
        let req: impl Iterator<Item = SearchByHandleReply> = sql::handles::Entity::find()
            .filter(sql::handles::Column::Handle.contains(handle))
            .all(&state.sql)
            .await?
            .into_iter()
            .map(|x| SearchByHandleReply {
                id: x.user_id,
                handle: x.handle,
            });

        return Ok(Json(req));
    }
    if let Some(user_id) = params.get("user_id") {
        let id = user_id.parse::<u64>()?;
        let req = sql::user::Entity::find_by_id(id).one(&state.sql).await?;

        return Ok(Json(req));
    }

    if let Some(tweet) = params.get("tweet") {
        let url = Url::parse(tweet)
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .path_segments()
            .ok_or(StatusCode::BAD_REQUEST)?
            .collect::<Vec<_>>();

        let tweetid = url
            .get(2)
            .ok_or(StatusCode::BAD_REQUEST)?
            .parse::<u64>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        state.couches.tweets.find(&FindQuery::)


    }

    Err(StatusCode::BAD_REQUEST)
}
