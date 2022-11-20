use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub sha_256_digest: Vec<u8>,
    pub large: bool,
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
