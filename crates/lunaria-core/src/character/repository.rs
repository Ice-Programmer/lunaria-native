use crate::character::error::CharacterError;
use crate::character::model::Character;
use async_trait::async_trait;

#[async_trait]
pub trait CharacterRepository: Send + Sync {
    async fn create(&self, character: Character) -> Result<(), CharacterError>;
}
