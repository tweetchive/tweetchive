use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "retweets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub retweeted_by: u64,
    pub retweet_of: u64,
    pub after_tweet_on_timeline: u64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
