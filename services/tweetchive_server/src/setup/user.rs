use couch_rs::types::view::CouchFunc;

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
