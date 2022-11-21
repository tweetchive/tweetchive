use crate::browser::USER_AGENT;
use crate::media::{download_image_media, download_video_media, upload};
use crate::AppState;
use ahash::{HashMap, HashMapExt, RandomState};
use chrono::Utc;
use dashmap::DashMap;
use futures::future::join_all;
use std::sync::Arc;
use tokio::join;
use tracing::{instrument, warn};
use tweetchive_core::rabbitmq::{ArchivedMedia, ArchivedUser, ArchivedUserData, MediaType};
use twtscrape::error::SResult;
use twtscrape::follow::{FollowType, Follows};
use twtscrape::search::Search;
use twtscrape::tweet::{Tweet, TweetType};
use twtscrape::user::User;
use twtscrape::usertweets::UserTweetsAndReplies;
use uuid::Uuid;

#[instrument]
pub async fn archive_user(state: Arc<AppState>, archive: Uuid, user: u64) -> SResult<()> {
    let scraper = state.anon_pool.get().await?;
    let user_archive = User::new(scraper.as_ref(), user).await?;
    let (followers, following) = join!(
        Follows::get_user_follow(scraper.as_ref(), user, FollowType::Followers),
        Follows::get_user_follow(scraper.as_ref(), user, FollowType::Following)
    );
    let followers = followers?;
    let following = following?;

    // get tweets
    let authenticated_scraper = state.account_pool.get().await?;
    let mut tweets = UserTweetsAndReplies::scroll_user_timeline(
        authenticated_scraper.as_ref(),
        user_archive.name.handle.clone(),
    )
    .await?;

    let mut tweet_map = Arc::new(DashMap::with_capacity_and_hasher(
        tweets.tweets.len(),
        RandomState::new(),
    ));
    let mut user_map = Arc::new(DashMap::with_capacity_and_hasher(
        tweets.users.len(),
        RandomState::new(),
    ));

    // pinned

    if let Some(pinned_id) = user_archive.pinned_tweet_id {
        match Tweet::parse_thread(scraper.as_ref(), pinned_id).await {
            Ok((mut t, u)) => {
                tweets.tweets.append(&mut t);
                for usr in u {
                    user_map.insert(usr.id, usr);
                }
            }
            Err(why) => {
                warn!(error = why, archive = archive, "Skipping...");
            }
        }
    }

    // tweets
    for tweet in tweets.tweets {
        // crawl the threads of these tweets
        match Tweet::parse_thread(scraper.as_ref(), tweet.id).await {
            Ok((t, u)) => {
                for twt in t {
                    // get quote tweet
                    match Search::make_query(
                        scraper.as_ref(),
                        format!("quoted_tweet_id:{}", twt.id),
                    )
                    .await
                    {
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
                    };
                    // get quoted tweet
                    if let TweetType::Tweet(data) = &twt {
                        if let Some(quoting_id) = data.reply_info.quoting {
                            match Tweet::parse_thread(scraper.as_ref(), quoting_id).await {
                                Ok((qt, qu)) => {
                                    for qt in qt {
                                        tweet_map.insert(qt.id, qt);
                                    }
                                    for qu in qu {
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
                    // add to list
                    tweet_map.insert(twt.id, twt);
                }
            }
            Err(why) => {
                warn!(error = why, archive = uuid, "Skipping...");
                continue;
            }
        }
    }

    for u in tweets.users {
        user_map.insert(u.id, u);
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
        .user_done_channel
        .sender
        .clone()
        .send_async(ArchivedUser {
            archival_id: archive,
            user: Ok(ArchivedUserData {
                user: user_archive,
                others: user_map.into_iter().map(|x| x.1).collect(),
                tweets: tweet_map.into_iter().map(|x| x.1).collect(),
                media: media_map.into_iter().map(|x| x.1).collect(),
                followers: followers.data,
                following: following.data,
            }),
            retrieved: Utc::now(),
        })
        .await?;
    Ok(())
}
