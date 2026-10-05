use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingError {
    #[error("unsupported platform: {0}")]
    UnsupportedPlatform(String),

    #[error("invalid key: {0}")]
    InvalidKey(String),

    #[error("shortcut storage operation failed: {0}")]
    Storage(String),
}

impl SettingError {
    pub fn storage(error: impl ToString) -> Self {
        Self::Storage(error.to_string())
    }
}
