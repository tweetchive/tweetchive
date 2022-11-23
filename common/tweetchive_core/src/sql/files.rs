use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique, indexed)]
    pub media_key: String,
    pub sea_hash: u64,
    pub large: bool,
    #[sea_orm(column_type = "Text")]
    pub original_url: String,
    #[sea_orm(column_type = "Text")]
    pub mime_type: String,
    #[sea_orm(column_type = "Text")]
    pub url: String,
    pub snapshot_id: Uuid,
}
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {
    Snapshot,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Snapshot => Entity::has_one(super::snapshots::Entity).into(),
        }
    }
}

impl Related<super::snapshots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Snapshot.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
