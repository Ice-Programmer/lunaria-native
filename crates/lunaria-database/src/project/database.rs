use super::project_meta;
use crate::connection::{
    check_integrity, connect, create_table, publish, set_database_metadata,
    validate_database_metadata,
};
use anyhow::{Context, Result};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QuerySelect, Set, TransactionTrait,
};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

pub(crate) const PROJECT_DATABASE_FILE: &str = "lunaria.db";
const PROJECT_DATABASE_ID: i64 = 0x4c554e41;
const PROJECT_SCHEMA_VERSION: i64 = 1;

pub(crate) struct ProjectDatabase {
    pub directory: PathBuf,
    pub connection: DatabaseConnection,
}

impl ProjectDatabase {
    pub(crate) async fn create(
        parent_directory: &Path,
        project_name: &str,
        created_at: i64,
    ) -> Result<Self> {
        let parent_directory = tokio::fs::canonicalize(parent_directory)
            .await
            .with_context(|| {
                format!(
                    "failed to resolve directory: {}",
                    parent_directory.display()
                )
            })?;
        let directory = parent_directory.join(project_name);

        tokio::fs::create_dir(&directory).await.map_err(|source| {
            let message = match source.kind() {
                ErrorKind::AlreadyExists => format!("file already exists: {}", directory.display()),
                ErrorKind::NotADirectory => {
                    format!("path is not a directory: {}", parent_directory.display())
                }
                _ => format!(
                    "failed to create project directory: {}",
                    directory.display()
                ),
            };
            anyhow::Error::new(source).context(message)
        })?;

        let directory = tokio::fs::canonicalize(&directory)
            .await
            .with_context(|| format!("failed to resolve directory: {}", directory.display()))?;

        match create_database_file(&directory, project_name, created_at).await {
            Ok(connection) => Ok(Self {
                directory,
                connection,
            }),
            Err(error) => {
                remove_created_project(&directory).await;
                Err(error)
            }
        }
    }

    pub(crate) fn directory(&self) -> &Path {
        &self.directory
    }

    pub(crate) async fn rollback(self) {
        let directory = self.directory.clone();
        let _ = self.connection.close().await;
        remove_created_project(&directory).await;
    }

    /// fetch opened project's database
    pub(crate) async fn open(path: &PathBuf) -> Result<Self> {
        let directory = tokio::fs::canonicalize(path)
            .await
            .with_context(|| format!("failed to resolve directory: {}", path.display()))?;

        let project_database_path = directory.join(PROJECT_DATABASE_FILE);
        let project_database_connection = connect(&project_database_path).await?;

        validate(&project_database_connection, &project_database_path).await?;

        Ok(Self {
            directory,
            connection: project_database_connection,
        })
    }

    pub(crate) async fn fetch_metadata(&self) -> Result<project_meta::Model> {
        project_meta::Entity::find_by_id(1)
            .one(&self.connection)
            .await?
            .with_context(|| {
                format!(
                    "invalid Lunaria database: {}",
                    self.directory.join(PROJECT_DATABASE_FILE).display()
                )
            })
    }
}

async fn create_database_file(
    directory: &Path,
    project_name: &str,
    created_at: i64,
) -> Result<DatabaseConnection> {
    let destination = directory.join(PROJECT_DATABASE_FILE);
    let temporary = NamedTempFile::new_in(directory)
        .with_context(|| {
            format!(
                "failed to create temporary database in {}",
                directory.display()
            )
        })?
        .into_temp_path();
    let database = connect(&temporary).await?;

    let result: Result<()> = async {
        let transaction = database.begin().await?;

        // 1. 创建所有表
        create_table(&transaction, project_meta::Entity).await?;
        create_table(&transaction, crate::character::model::Entity).await?;

        project_meta::ActiveModel {
            id: Set(1),
            name: Set(project_name.to_owned()),
            created_at: Set(created_at),
        }
        .insert(&transaction)
        .await?;

        set_database_metadata(&transaction, PROJECT_DATABASE_ID, PROJECT_SCHEMA_VERSION).await?;
        transaction.commit().await?;
        Ok(())
    }
    .await;

    database.close().await?;
    result?;
    publish(temporary, &destination)?;

    let database = connect(&destination).await?;
    if let Err(error) = validate(&database, &destination).await {
        database.close().await?;
        return Err(error);
    }

    Ok(database)
}

async fn validate(database: &DatabaseConnection, path: &Path) -> Result<()> {
    validate_database_metadata(database, path, PROJECT_DATABASE_ID, PROJECT_SCHEMA_VERSION).await?;

    project_meta::Entity::find()
        .limit(0)
        .all(database)
        .await
        .with_context(|| format!("invalid Lunaria database: {}", path.display()))?;
    check_integrity(database).await
}

async fn remove_created_project(directory: &Path) {
    let _ = tokio::fs::remove_file(directory.join(PROJECT_DATABASE_FILE)).await;
    let _ = tokio::fs::remove_dir(directory).await;
}
