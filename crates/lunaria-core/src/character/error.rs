use thiserror::Error;

#[derive(Debug, Error)]
pub enum CharacterError {
    #[error("character storage operation failed: {0}")]
    Storage(String),
}

impl CharacterError {
    pub fn storage(error: impl ToString) -> Self {
        Self::Storage(error.to_string())
    }
}
