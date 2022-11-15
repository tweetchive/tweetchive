use actix::Message;

#[derive(Message)]
pub struct QueryUserProfileMessage(String);
