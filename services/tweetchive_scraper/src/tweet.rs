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

#[instrument]
pub async fn archive_tweet(
    scraper: &Scraper,
    tweet_id: impl TwitterIdType + Display,
    tweets: Arc<DashMap<u64, Tweet, RandomState>>,
    users: Arc<DashMap<u64, User, RandomState>>,
) -> SResult<()> {
    let (mut first_twt, mut first_usr) = Tweet::parse_thread(scraper, tweet_id).await?;
    
    
    for ftwts in first_twt {
        if let TweetType::Tweet(data) = &ftwts {
            if let Some(quote) = data.reply_info.quoting {
                let (quoted_twts, quoted_users) = Tweet::parse_thread(scraper, quote).await?;
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
