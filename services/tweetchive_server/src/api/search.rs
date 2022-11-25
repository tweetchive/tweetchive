use crate::api::tweet::tweet_api_req_url;
use crate::api::user::user_api_req_url;
use crate::AppState;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::couchdb::tweet::Tweet;
use tweetchive_core::sql;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct SearchResponseItem {
    pub item_id: u64,
    #[serde(flatten)]
    pub additional_info: HashMap<String, String>,
    pub pointer: String,
}

#[instrument]
pub async fn search(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if let Some(handle) = params.get("handle") {
        let req: impl Iterator<Item = SearchResponseItem> = sql::handles::Entity::find()
            .filter(sql::handles::Column::Handle.contains(handle))
            .all(&state.sql)
            .await?
            .into_iter()
            .map(|x| SearchResponseItem {
                item_id: x.user_id,
                additional_info: HashMap::from([
                    ("handle".to_string(), x.handle),
                    ("snapshot".to_string(), x.snapshot_id.to_string()),
                ]),
                pointer: user_api_req_url(x.user_id),
            });

        return Ok(Json(req));
    }
    if let Some(user_id) = params.get("user_id") {
        let id = user_id.parse::<u64>()?;
        return match sql::user::Entity::find_by_id(id).one(&state.sql).await? {
            Some(usr) => {
                let sri = SearchResponseItem {
                    item_id: usr.id,
                    additional_info: HashMap::from([
                        ("handle".to_string(), usr.latest_handle),
                        ("snapshot".to_string(), usr.latest_snapshot_id.to_string()),
                    ]),
                    pointer: user_api_req_url(usr.id),
                };
                Ok(Json(vec![sri]))
            }
            None => Ok(Json(())),
        };
    }

    if let Some(tweet) = params.get("tweet_id") {
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

        let document = state
            .couches
            .tweets
            .get::<Tweet>(url.get(2).unwrap())
            .await?;

        let tweet = SearchResponseItem {
            item_id: tweetid,
            additional_info: HashMap::from([(
                "conversation".to_string(),
                document.conversation_id.to_string(),
            )]),
            pointer: tweet_api_req_url(tweetid),
        };

        return Ok(Json(vec![tweet]));
    }

    Err(StatusCode::BAD_REQUEST)
}
