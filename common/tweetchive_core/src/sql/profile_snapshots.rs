use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "profile_snapshot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user: u64,
    #[sea_orm(indexed, column_type = "Text")]
    pub display: String,
    #[sea_orm(indexed, column_type = "Text")]
    pub handle: String,
    #[sea_orm(column_type = "Text")]
    pub bio: String,
    #[sea_orm(nullable, column_type = "Text")]
    pub profession: Option<String>,
    pub joined: DateTime,
    pub birthday: DateTime,
    pub pfp: Uuid,
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
