use crate::db_access::tweet::tweet;
use crate::db_access::user::{user, UserData};
use crate::AppState;
use axum::extract::Path;
use axum::response::{IntoResponse, Redirect};
use axum::{Extension, Form};
use sailfish::TemplateOnce;
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;
use tweetchive_core::couchdb::tweet::Tweet;
use url::{ParseError, Url};

#[derive(Deserialize)]
struct UserResults {
    pub item_id: u64,
    pub handle: String,
}

#[derive(Deserialize)]
struct TweetResults {
    pub item_id: u64,
    pub link: String,
}

#[derive(TemplateOnce)]
#[template(path = "search.stpl")]
struct SearchTemplate {
    pub user_results: Vec<UserResults>,
    pub tweet_results: Vec<TweetResults>,
}

#[derive(Deserialize)]
struct SearchQuery {
    pub search: String,
}

#[instrument]
pub async fn search(
    Extension(state): Extension<Arc<AppState>>,
    Form(search): Form<SearchQuery>,
) -> impl IntoResponse {
    // see if this is a URL
    if let Ok(url) = Url::parse(&search.search) {
        return Ok(Redirect::to(url.path()));
    }

    if let Ok(id) = str::parse::<u64>(&search.search) {
        // tweet results
        let tweets = match tweet(state.clone(), id).await? {
            Some(t) => vec![t],
            None => vec![],
        };
        let user = match user(state.clone(), id, None).await? {
            None => vec![],
            Some(u) => vec![u],
        };
    }
}
