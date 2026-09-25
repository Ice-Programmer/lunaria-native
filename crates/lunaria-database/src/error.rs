use std::{io, path::PathBuf, time::SystemTimeError};

use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database operation failed: {0}")]
    Database(#[from] DbErr),

    #[error("I/O operation failed for {path}: {source}")]
    IO {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("file already exists: {0}")]
    AlreadyExists(PathBuf),

    #[error("path is not a directory: {0}")]
    PathNotDirectory(PathBuf),

    #[error("invalid Lunaria database: {0}")]
    InvalidDatabase(PathBuf),

    #[error("unsupported database schema version {version}: {path}")]
    UnsupportedSchemaVersion { path: PathBuf, version: i64 },

    #[error("database integrity check failed: {0}")]
    IntegrityCheckFailed(String),

    #[error("system clock is earlier than the Unix epoch: {0}")]
    SystemTime(#[from] SystemTimeError),

    #[error("Unix timestamp is outside the supported range")]
    TimestampOutOfRange,
}

impl DatabaseError {
    pub(crate) fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::IO {
            path: path.into(),
            source,
        }
    }
}
