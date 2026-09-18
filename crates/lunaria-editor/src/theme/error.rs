#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("theme not found: {name}")]
    NotFound { name: String },

    #[error("failed to load themes: {err}")]
    LoadFailed { err: String },
}
