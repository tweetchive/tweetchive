use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub tweet_id: u64,
    pub user: u64,
    pub date: DateTime,

    pub reply_to: Option<u64>,
    pub quote_retweet: Option<u64>,

    #[sea_orm(column_type = "Text")]
    pub content: String,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
