use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "authenticated")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub github_name: String,
    pub email: String,
    pub is_admin: bool,
    pub profile_picture: String,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {
    Token,
    Report
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Token => Entity::has_many(super::token::Entity).into(),
            Relation::Report => Entity::has_many(super::report::Entity).into(),
        }
    }
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}
impl Related<super::report::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Report.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
