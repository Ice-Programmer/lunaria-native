use lunaria_core::character::model::Character;
use sea_orm::entity::prelude::*;
use std::path::PathBuf;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "character")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    #[sea_orm(unique)]
    pub code: String,
    pub description: String,
    pub avatar_path: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Character {
    fn from(model: Model) -> Self {
        Self {
            name: model.name,
            code: model.code,
            description: model.description,
            avatar_path: model.avatar_path.map(PathBuf::from),
        }
    }
}

impl TryFrom<Character> for ActiveModel {
    type Error = anyhow::Error;

    fn try_from(character: Character) -> anyhow::Result<Self> {
        let avatar_path = character
            .avatar_path
            .map(|path| path.into_os_string().into_string())
            .transpose()
            .map_err(|_| anyhow::anyhow!("avatar path is not valid UTF-8"))?;

        Ok(Self {
            name: Set(character.name),
            code: Set(character.code),
            description: Set(character.description),
            avatar_path: Set(avatar_path),
            ..Default::default()
        })
    }
}
