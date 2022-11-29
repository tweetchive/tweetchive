use couch_rs::types::view::CouchFunc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const USER_BY_HANDLE: &str = "user_by_handle";

pub fn user_by_handle() -> CouchFunc {
    CouchFunc {
        map: "\
        function (doc) {\
            var max = 0;
            var value = null;
            for (var key in doc.data) {
                if key > max {
                    max = key;
                    value = doc.data[max];
                }
            }
            emit(value.name.handler, value)
        }"
        .to_string(),
        reduce: None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct BasicInfoById {
    pub display: String,
    pub handle: String,
    pub profile_picture: Uuid,
    pub banner: Uuid,
    pub nft: bool,
}

pub const BASIC_INFO_ID: &str = "user_basic_info_id";

pub fn basic_info_by_id() -> CouchFunc {
    CouchFunc {
        map: "\
        function (doc) {
            var max = 0;
            var value = null;
            for (var key in doc.data) {
                if key > max {
                    max = key;
                    value = doc.data[max];
                }
            }
            emit(value.id, {
                \"display\": value.name.display,
                \"handler\": value.name.handler,
                \"profile_picture\": doc.media[max].profile_picture,
                \"banner\": doc.media[max].banner,
                \"nft\": doc.media[max].is_nft,
            })
        }
        "
        .to_string(),
        reduce: None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct BasicInfoByHandle {
    pub id: u64,
    pub display: String,
    pub profile_picture: Uuid,
    pub banner: Uuid,
    pub nft: bool,
}

pub const BASIC_INFO_HANDLE: &str = "user_basic_info_handle";

pub fn basic_info_by_handle() -> CouchFunc {
    CouchFunc {
        map: "\
        function (doc) {
            var max = 0;
            var value = null;
            for (var key in doc.data) {
                if key > max {
                    max = key;
                    value = doc.data[max];
                }
            }
            emit(value.name.handler, {
                \"id\": value.id,
                \"display\": value.name.display,
                \"profile_picture\": doc.media[max].profile_picture,
                \"banner\": doc.media[max].banner,
                \"nft\": doc.media[max].is_nft,
            })
        }
        "
        .to_string(),
        reduce: None,
    }
}
