use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub reporter: i64,
    pub report_time: DateTime<Utc>,
    #[sea_orm(indexed)]
    pub reporting_type: String,
    pub reporting: u64,
    pub reporting_snapshot: Uuid,
    #[sea_orm(indexed)]
    pub report_type: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub report_additional_info: Option<String>,
    pub resolved: bool,
    #[sea_orm(indexed, nullable)]
    pub resolution: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub resolution_reason: Option<String>,
    pub resolution_time: DateTime<Utc>,
    pub resolver: i64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {
    AuthenticatedReporter,
    Snapshot,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::AuthenticatedReporter => Entity::belongs_to(super::authenticated::Entity)
                .from(Column::Reporter)
                .to(super::authenticated::Column::Id)
                .into(),
            Relation::Snapshot => Entity::has_one(super::snapshots::Entity).into(),
        }
    }
}

impl Related<super::authenticated::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthenticatedReporter.def()
    }
}

impl Related<super::snapshots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Snapshot.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
