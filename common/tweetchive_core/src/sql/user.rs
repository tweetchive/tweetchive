use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {
    Snapshot,
    Handles,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Snapshot => Entity::has_one(super::snapshots::Entity).into(),
            Relation::Handles => Entity::has_many(super::handles::Entity).into(),
        }
    }
}

impl Related<super::snapshots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Snapshot.def()
    }
}

impl Related<super::handles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Handles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
