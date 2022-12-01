use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "snapshots")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub start: DateTime<Utc>,
    #[sea_orm(nullable)]
    pub finish: Option<DateTime<Utc>>,
    #[sea_orm(indexed, column_type = "Text")]
    pub init_args: String,
    pub started_by: String,
    pub priority: i32,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {
    Files,
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> Relation {
        match self {
            Relation::Files => Entity::belongs_to(super::files::Entity)
                .from(Column::Id)
                .to(super::files::Column::SnapshotId)
                .into(),
            Relation::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Id)
                .to(super::user::Column::SnapshotId)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
