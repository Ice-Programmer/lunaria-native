use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("project name is required")]
    NameRequired,

    #[error("project name must not exceed {max} characters")]
    NameTooLong { max: usize },

    #[error("project name contains characters that cannot be used in a directory name")]
    InvalidName,

    #[error("project path is required")]
    PathRequired,

    #[error("project storage operation failed: {0}")]
    Storage(String),
}

impl ProjectError {
    pub fn storage(error: impl ToString) -> Self {
        Self::Storage(error.to_string())
    }
}
