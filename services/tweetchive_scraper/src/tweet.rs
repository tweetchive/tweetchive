use crate::browser::USER_AGENT;
use crate::media::{download_image_media, download_video_media, upload};
use crate::AppState;
use ahash::RandomState;
use chrono::Utc;
use dashmap::DashMap;
use futures::future::join_all;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use tracing::instrument;
use tracing::log::warn;
use tweetchive_core::rabbitmq::{ArchivedMedia, ArchivedTweetData, ArchivedTweets, MediaType};
use twtscrape::error::SResult;
use twtscrape::scrape::Scraper;
use twtscrape::search::Search;
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
                    tweet_map.insert(twt.id, twt);
                }
                for u in quoted_users {
                    user_map.insert(u.id, u);
                }
            }
        }

        // get quote tweets

        match Search::make_query(scraper.as_ref(), format!("quoted_tweet_id:{}", ftwts.id)).await {
            Ok(qrt) => {
                for q in join_all(
                    qrt.tweets
                        .into_iter()
                        .map(|twt| Tweet::parse_thread(scraper.as_ref(), twt)),
                )
                .await
                {
                    match q {
                        Ok((qrt_twt, qrt_usr)) => {
                            for qt in qrt_twt {
                                tweet_map.insert(qt.id, qt);
                            }
                            for qu in qrt_usr {
                                user_map.insert(qu.id, qu);
                            }
                        }
                        Err(why) => {
                            warn!(error = why, archive = archive, "Skipping...");
                            continue;
                        }
                    }
                }
            }
            Err(why) => {
                warn!(error = why, archive = archive, "Skipping...");
                continue;
            }
        }

        tweet_map.insert(ftwts.id, ftwts);
    }

    for usrs in first_usr {
        user_map.insert(usrs.id, usrs);
    }

    let mut media_map = HashMap::new();

    for a in tweet_map.iter() {
        if let TweetType::Tweet(data) = &a {
            for media in &data.entry.media {
                if let None = media_map.get(&media.media_key) {
                    let media_type = media.r#type.to_lowercase();
                    let dlinfo: (String, impl AsRef<[u8]>) = if media_type.contains("video")
                        || media_type.contains("gif")
                    {
                        let download =
                            match download_video_media(USER_AGENT, &media.expanded_url).await {
                                Ok(dl) => dl,
                                Err(why) => {
                                    warn!(
                                        media = media_type,
                                        url = media.media_key,
                                        error = why,
                                        "Error Downloading"
                                    );
                                    continue;
                                }
                            };
                        (download.content_type, download.data)
                    } else if media_type.contains("picture") || media_type.contains("image") {
                        let download =
                            match download_image_media(scraper.as_ref(), &media.expanded_url).await
                            {
                                Ok(dl) => dl,
                                Err(why) => {
                                    warn!(
                                        media = media_type,
                                        url = media.media_key,
                                        error = why,
                                        "Error Downloading"
                                    );
                                    continue;
                                }
                            };
                        (download.content_type, download.data)
                    } else {
                        warn!(
                            media = media_type,
                            url = media.media_key,
                            "Unknown Media Type, Skipping"
                        );
                        continue;
                    };
                    if let Err(_) =
                        upload(state.clone(), &media.media_key, &dlinfo.0, dlinfo.1).await
                    {
                        warn!(media = media_type, url = media.media_key, "Error Uploading");
                        continue;
                    }

                    let media_archive = ArchivedMedia {
                        archival_id: archive,
                        media_id: media.media_key.clone(),
                        media_type: if media_type.contains("video") {
                            MediaType::Video
                        } else {
                            MediaType::Gif
                        },
                        content_type: dlinfo.0,
                        retrieved: Utc::now(),
                    };
                    media_map.insert(media.media_key.clone(), media_archive);
                }
            }
        }
    }

    state
        .tweet_done_channel
        .sender
        .clone()
        .send_async(ArchivedTweets {
            archival_id,
            tweets: Ok(ArchivedTweetData {
                tweets: tweet_map.into_iter().map(|x| x.1).collect(),
                users: user_map.into_iter().map(|x| x.1).collect(),
                media: media_map.into_iter().map(|x| x.1).collect(),
            }),
            retrieved: Utc::now(),
        })
        .await?;
    Ok(())
}
