use crate::AppState;
use argon2::password_hash::SaltString;
use argon2::{Argon2, Pass, PasswordHash, PasswordHasher, PasswordVerifier};
use color_eyre::Result;
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::sql::{authenticated, token};
use url::Url;

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

#[instrument]
pub async fn user_by_token(
    state: Arc<AppState>,
    token: String,
) -> Result<Option<authenticated::Model>> {
    let tkn_split = token.split(".").collect::<Vec<&str>>();
    let id = match tkn_split.get(0).map(|x| x.parse::<i64>().ok()).flatten() {
        Some(id) => id,
        None => return Ok(None),
    };

    let token = match tkn_split.get(1).map(|x| base64::decode(x).ok()).flatten() {
        Some(t) => t,
        None => return Ok(None),
    };

    let auth = match token::Entity::find_by_id(salt).one(&state.sql).await? {
        Some(a) => a,
        None => return Ok(None),
    };

    if id != auth.user {
        return Ok(None);
    }

    let argon2 = Argon2::default();
    let password = PasswordHash::new(&auth.id)?;
    argon2.verify_password(&token, &password)?;
    Ok(authenticated::Entity::find_by_id(auth.user)
        .one(&state.sql)
        .await?)
}

#[derive(Deserialize)]
struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: Url,
    pub email: String,
}

pub async fn register_or_update_user(
    state: Arc<AppState>,
    auth_token: String,
) -> Result<authenticated::Model> {
    let auth_user = reqwest::Client::default()
        .get("https://api.github.com/user")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", auth_token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "tweetchive.org (github.com/tweetchive)")
        .send()
        .await?
        .json::<User>()
        .await?;

    // check exists
    match authenticated::Entity::find_by_id(auth_user.id)
        .one(&state.sql)
        .await?
    {
        Some(already_exists) => {
            if (auth_user.avatar_url.to_string() != already_exists.profile_picture)
                || (auth_user.email != already_exists.email)
            {
                let mut active = already_exists.into_active_model();
                active.email = ActiveValue::Set(auth_user.email);
                active.profile_picture = ActiveValue::Set(auth_user.avatar_url.to_string());
                let new = active.update(&state.sql).await?;
                Ok(new)
            } else {
                Ok(already_exists)
            }
        }
        None => {
            let admin = state
                .config
                .read()
                .await
                .admin_github_users
                .contains(&auth_user.login);

            let new_user = authenticated::ActiveModel {
                id: ActiveValue::Set(auth_user.id),
                github_name: ActiveValue::Set(auth_user.login),
                email: ActiveValue::Set(auth_user.email),
                is_admin: ActiveValue::Set(admin),
                profile_picture: ActiveValue::Set(auth_user.avatar_url.to_string()),
            };

            let insert = new_user.insert(&state.sql).await?;
            Ok(insert)
        }
    }
}

#[instrument]
pub async fn issue_new_token(state: Arc<AppState>, user: i64) -> Result<String> {
    let mut data = vec![0; 64];
    ChaCha20Rng::from_entropy().try_fill_bytes(&mut data)?;
    let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let argon = Argon2::default();
    let hash = argon.hash_password(&data, &salt)?.to_string();

    // insert this into the database
    let new_token = token::ActiveModel {
        id: ActiveValue::Set(hash),
        user: ActiveValue::Set(user),
    };

    new_token.insert(&state.sql).await?;

    Ok(format!("{user}.{}", base64::encode(data)))
}

#[instrument]
pub async fn burn_token(state: Arc<AppState>, token: String) -> Result<()> {
    token::Entity::delete_by_id(token).exec(&state.sql).await?;

    Ok(())
}

#[instrument]
pub async fn burn_all_tokens(state: Arc<AppState>, user: i64) -> Result<()> {
    token::Entity::delete_many()
        .filter(token::Column::User.eq(user))
        .exec(&state.sql)
        .await?;

    Ok(())
}
