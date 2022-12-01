use crate::herr::HResult;
use crate::AppState;
use axum::http::StatusCode;
use axum::Extension;
use color_eyre::Result;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::{couchdb, sql};
use twtscrape::user::User;
use uuid::Uuid;

#[instrument]
pub async fn user(
    state: Arc<AppState>,
    id: u64,
    snapshot: Option<Uuid>,
) -> Result<Option<UserData>> {
    match snapshot {
        Some(u) => {
            let resp = get_user_data(state.clone(), id, u).await?;
            match resp {
                Some(d) => Ok(Some(d)),
                None => Ok(None),
            }
        }
        None => match sql::user::Entity::find_by_id(id).one(&state.sql).await? {
            Some(u) => {
                let resp = get_user_data(state.clone(), id, u.latest_snapshot_id).await?;
                match resp {
                    Some(d) => Ok(Some(d)),
                    None => Ok(None),
                }
            }
            None => Ok(None),
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub id: u64,
    pub user: User,
}

#[instrument]
async fn get_user_data(state: Arc<AppState>, id: u64, snapshot: Uuid) -> Result<Option<UserData>> {
    let mut user = state
        .couches
        .tweets
        .get::<couchdb::user::UserArchive>(&id.to_string())
        .await?;

    let data = match user.data.remove(&snapshot) {
        Some(ud) => UserData { id, user: ud },
        None => return Ok(None),
    };

    Ok(Some(data))
}
