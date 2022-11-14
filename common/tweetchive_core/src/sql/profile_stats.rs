use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "profile_stats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user: u64,

    pub following: u64,
    pub followers: u64,

    pub snapshot_id: u64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
