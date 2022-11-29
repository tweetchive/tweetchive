use couch_rs::types::view::CouchFunc;
use serde::{Deserialize, Serialize};
use twtscrape::tweet::TweetData;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LatestTweetsOfUserOutput {
    pub _id: String,
    pub also_ids: Vec<String>,
    pub conversation_id: u64,
    pub reply_to: u64,
    pub data: TweetData,
    pub snapshot: Uuid,
}

pub const TWEET_LATEST: &str = "latest_tweet_of_user";

pub fn latest_tweets_of_user() -> CouchFunc {
    CouchFunc {
        map: format!(
            "\
        function (doc) {{\
            var max = 0;
            var value = null;
            for (var key in doc.data) {{
                if key > max {{
                    max = key;
                    value = doc.data[max];
                }}
            }}
            emit(doc.author, {{
                \"_id\": doc._id,
                \"also_ids\": doc.also_ids
                \"conversation_id\": doc.conversation_id,
                \"reply_to\": doc.reply_to,
                \"data\": value
                \"snapshot\": max
            }})
        }} "
        ),
        reduce: None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct LatestTweetsOfConversationOutput {
    pub _id: String,
    pub also_ids: Vec<String>,
    pub author: u64,
    pub reply_to: u64,
    pub data: TweetData,
    pub snapshot: Uuid,
}

pub const CONVERSATION_LATEST: &str = "latest_tweet_of_conversation";

pub fn latest_tweets_of_conversation() -> CouchFunc {
    CouchFunc {
        map: format!(
            "\
        function (doc) {{\
            var max = 0;
            var value = null;
            for (var key in doc.data) {{
                if key > max {{
                    max = key;
                    value = doc.data[max];
                }}
            }}
            emit(doc.conversation_id, {{
                \"_id\": doc._id,
                \"also_ids\": doc.also_ids
                \"author\": doc.author,
                \"reply_to\": doc.reply_to,
                \"data\": value
                \"snapshot\": max
            }})
        }} "
        ),
        reduce: None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct LatestRepliesToTweetOutput {
    pub _id: String,
    pub also_ids: Vec<String>,
    pub author: u64,
    pub conversation_id: u64,
    pub data: TweetData,
    pub snapshot: Uuid,
}

pub const REPLY_LATEST: &str = "latest_replies_of_tweet";

pub fn latest_replies_of_tweet() -> CouchFunc {
    CouchFunc {
        map: format!(
            "\
        function (doc) {{\
            var max = 0;
            var value = null;
            for (var key in doc.data) {{
                if key > max {{
                    max = key;
                    value = doc.data[max];
                }}
            }}
            emit(doc.reply_to, {{
                \"_id\": doc._id,
                \"also_ids\": doc.also_ids
                \"author\": doc.author,
                \"conversation_id\": doc.conversation_id,
                \"data\": value
                \"snapshot\": max
            }})
        }} "
        ),
        reduce: None,
    }
}
