use crate::DatabaseManager;
use crate::character::character;
use async_trait::async_trait;
use lunaria_core::character::error::CharacterError;
use lunaria_core::character::model::Character;
use lunaria_core::character::repository::CharacterRepository;
use sea_orm::{ActiveModelTrait, Set};
use std::sync::Arc;

pub struct Repository {
    databases: Arc<DatabaseManager>,
}

impl Repository {
    pub fn new(databases: Arc<DatabaseManager>) -> Arc<Self> {
        Arc::new(Self { databases })
    }
}

#[async_trait]
impl CharacterRepository for Repository {
    async fn create(&self, character: Character) -> Result<(), CharacterError> {
        let project = self.databases.current_project().await?;

        let avatar_path = character
            .avatar_path
            .map(|path| {
                path.into_os_string()
                    .into_string()
                    .map_err(|_| CharacterError::storage("avatar path is not valid UTF-8"))
            })
            .transpose()?;

        character::ActiveModel {
            name: Set(character.name),
            code: Set(character.code),
            description: Set(character.description),
            avatar_path: Set(avatar_path),
            ..Default::default()
        }
        .insert(&project.connection)
        .await
        .map_err(CharacterError::storage)?;

        Ok(())
    }
}
