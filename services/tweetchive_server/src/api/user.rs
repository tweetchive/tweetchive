use crate::api::SnapshotTag;
use crate::AppState;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use color_eyre::Result;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::sql::user::Model;
use tweetchive_core::{couchdb, sql};
use twtscrape::user::User;
use uuid::Uuid;

pub fn user_api_req_url(user_id: u64) -> String {
    format!("api/user/{user_id}/latest")
}

#[instrument]
pub async fn user(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u64>,
    Path(snapshot): Path<SnapshotTag>,
) -> impl IntoResponse {
    match snapshot {
        SnapshotTag::Uuid(u) => {
            let resp = get_user_data(state.clone(), id, u).await?;
            match resp {
                Some(d) => Ok(Json(d)),
                None => Err(StatusCode::NOT_FOUND),
            }
        }
        SnapshotTag::String(s) => {
            if s != "latest" {
                return Err(StatusCode::BAD_REQUEST);
            }

            match sql::user::Entity::find_by_id(id).one(&state.sql).await? {
                Some(u) => {
                    let resp = get_user_data(state.clone(), id, u.latest_snapshot_id).await?;
                    match resp {
                        Some(d) => Ok(Json(d)),
                        None => Err(StatusCode::NOT_FOUND),
                    }
                }
                None => Err(StatusCode::NOT_FOUND),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub id: u64,
    pub user: User,
    pub tweets: String,
}

#[instrument]
async fn get_user_data(state: Arc<AppState>, id: u64, snapshot: Uuid) -> Result<Option<UserData>> {
    let mut user = state
        .couches
        .tweets
        .get::<couchdb::user::UserArchive>(&id.to_string())
        .await?;

    let data = match user.data.remove(&snapshot) {
        Some(ud) => UserData {
            id,
            user: ud,
            tweets: tw,
        },
        None => return Ok(None),
    };

    Ok(Some(data))
}
