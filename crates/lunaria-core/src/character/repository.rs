use crate::character::model::Character;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CharacterRepository: Send + Sync {
    async fn create(&self, character: Character) -> Result<()>;
}
