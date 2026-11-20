use crate::character::model::Character;
use crate::character::repository::CharacterRepository;
use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct CharacterService {
    repository: Arc<dyn CharacterRepository>,
}

impl CharacterService {
    pub fn new(repository: Arc<dyn CharacterRepository>) -> Self {
        Self { repository }
    }

    pub async fn list(&self) -> Result<Vec<Character>> {
        self.repository.list().await
    }
}