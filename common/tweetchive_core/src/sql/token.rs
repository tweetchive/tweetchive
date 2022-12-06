use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    // phc string token
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user: i64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {
    Authenticated,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Authenticated => Entity::belongs_to(super::authenticated::Entity)
                .from(super::authenticated::Column::Id)
                .to(Column::User)
                .into(),
        }
    }
}

impl Related<super::authenticated::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Authenticated.def()
    }
}