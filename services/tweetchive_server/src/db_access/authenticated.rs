use crate::AppState;
use color_eyre::Result;
use sea_orm::EntityTrait;
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::sql::authenticated;

#[instrument]
pub async fn check_user_exists(
    state: Arc<AppState>,
    github_id: i64,
) -> Result<Option<authenticated::Model>> {
    authenticated::Entity::find_by_id(github_id)
        .one(&state.sql)
        .await
        .into()
}

pub async fn user_authentication(
    state: Arc<AppState>,
    token: String,
) -> Result<Option<authenticated::Model>> {
}
