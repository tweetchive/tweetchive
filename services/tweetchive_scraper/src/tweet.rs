use crate::AppState;
use ahash::RandomState;
use dashmap::DashMap;
use std::fmt::Display;
use std::sync::Arc;
use tracing::instrument;
use twtscrape::error::SResult;
use twtscrape::scrape::Scraper;
use twtscrape::tweet::{Tweet, TweetType};
use twtscrape::user::User;
use twtscrape::TwitterIdType;
use uuid::Uuid;

#[instrument]
pub async fn archive_tweet(state: Arc<AppState>, archival_id: Uuid, tweet_id: u64) -> SResult<()> {
    let scraper = state.account_pool.get().await?;

    let mut tweet_map = Arc::new(DashMap::with_capacity_and_hasher(
        tweets.tweets.len(),
        RandomState::new(),
    ));
    let mut user_map = Arc::new(DashMap::with_capacity_and_hasher(
        tweets.users.len(),
        RandomState::new(),
    ));

    let (mut first_twt, mut first_usr) = Tweet::parse_thread(scraper.as_ref(), tweet_id).await?;

    for ftwts in first_twt {
        if let TweetType::Tweet(data) = &ftwts {
            if let Some(quote) = data.reply_info.quoting {
                let (quoted_twts, quoted_users) =
                    Tweet::parse_thread(scraper.as_ref(), quote).await?;
                for twt in quoted_twts {
                    tweets.insert(twt.id, twt);
                }
                for u in quoted_users {
                    users.insert(u.id, u);
                }
            }
        }

        tweets.insert(ftwts.id, ftwts);
    }

    for usrs in first_usr {
        users.insert(usrs.id, usrs);
    }

    Ok(())
}
