use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "recent_project")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub project_name: String,

    #[sea_orm(unique)]
    pub project_path: String,

    pub created_at: i64,

    pub last_opened_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
