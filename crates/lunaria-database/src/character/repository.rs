use crate::DatabaseManager;
use crate::character::model;
use anyhow::Result;
use async_trait::async_trait;
use lunaria_core::character::model::Character;
use lunaria_core::character::repository::CharacterRepository;
use lunaria_di::{SourceKind, repository};
use sea_orm::{ActiveModelTrait, EntityTrait};
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
#[repository(key = SourceKind::Local)]
impl CharacterRepository for Repository {
    async fn create(&self, character: Character) -> Result<()> {
        let project_db = self.databases.current_project().await?;

        model::ActiveModel::try_from(character)?
            .insert(&project_db.connection)
            .await?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Character>> {
        let project_db = self.databases.current_project().await?;

        let character_list = model::Entity::find()
            .order_by_id_asc()
            .all(&project_db.connection)
            .await?
            .into_iter()
            .map(Character::from)
            .collect::<Vec<Character>>();

        Ok(character_list)
    }
}
