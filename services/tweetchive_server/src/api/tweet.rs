pub fn tweet_api_req_url(tweet_id: u64) -> String {
    format!("api/status/{tweet_id}/latest")
}
