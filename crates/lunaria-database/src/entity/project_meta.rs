use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "project_meta")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, extra = "CHECK (id = 1)")]
    pub id: i64,
    pub name: String,
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
