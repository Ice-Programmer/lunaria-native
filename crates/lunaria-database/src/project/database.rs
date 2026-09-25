use super::project_meta;
use crate::DatabaseError;
use crate::connection::{
    check_integrity, connect, create_table, publish, set_database_metadata,
    validate_database_metadata,
};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QuerySelect, Set, TransactionTrait,
};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

const PROJECT_DATABASE_FILE: &str = "lunaria.db";
const PROJECT_DATABASE_ID: i64 = 0x4c554e41;
const PROJECT_SCHEMA_VERSION: i64 = 1;

pub(crate) struct ProjectDatabase {
    directory: PathBuf,
    connection: DatabaseConnection,
}

impl ProjectDatabase {
    pub(crate) async fn create(
        parent_directory: &Path,
        project_name: &str,
        created_at: i64,
    ) -> Result<Self, DatabaseError> {
        let metadata = tokio::fs::metadata(parent_directory)
            .await
            .map_err(|source| DatabaseError::io(parent_directory, source))?;
        if !metadata.is_dir() {
            return Err(DatabaseError::PathNotDirectory(
                parent_directory.to_path_buf(),
            ));
        }

        let parent_directory = tokio::fs::canonicalize(parent_directory)
            .await
            .map_err(|source| DatabaseError::io(parent_directory, source))?;
        let directory = parent_directory.join(project_name);

        if tokio::fs::try_exists(&directory)
            .await
            .map_err(|source| DatabaseError::io(&directory, source))?
        {
            return Err(DatabaseError::AlreadyExists(directory));
        }

        tokio::fs::create_dir(&directory)
            .await
            .map_err(|source| DatabaseError::io(&directory, source))?;

        let directory = tokio::fs::canonicalize(&directory)
            .await
            .map_err(|source| DatabaseError::io(&directory, source))?;

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
}

async fn create_database_file(
    directory: &Path,
    project_name: &str,
    created_at: i64,
) -> Result<DatabaseConnection, DatabaseError> {
    let destination = directory.join(PROJECT_DATABASE_FILE);
    let temporary = NamedTempFile::new_in(directory)
        .map_err(|source| DatabaseError::io(directory, source))?
        .into_temp_path();
    let database = connect(&temporary).await?;

    let result: Result<(), DatabaseError> = async {
        let transaction = database.begin().await?;
        create_table(&transaction, project_meta::Entity).await?;

        project_meta::ActiveModel {
            id: Set(1),
            name: Set(project_name.to_owned()),
            created_at: Set(created_at),
        }
        .insert(&transaction)
        .await?;

        set_database_metadata(&transaction, PROJECT_DATABASE_ID, PROJECT_SCHEMA_VERSION).await?;
        check_integrity(&transaction).await?;
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

async fn validate(database: &DatabaseConnection, path: &Path) -> Result<(), DatabaseError> {
    validate_database_metadata(database, path, PROJECT_DATABASE_ID, PROJECT_SCHEMA_VERSION).await?;

    project_meta::Entity::find()
        .limit(0)
        .all(database)
        .await
        .map_err(|_| DatabaseError::InvalidDatabase(path.to_path_buf()))?;
    check_integrity(database).await
}

async fn remove_created_project(directory: &Path) {
    let _ = tokio::fs::remove_file(directory.join(PROJECT_DATABASE_FILE)).await;
    let _ = tokio::fs::remove_dir(directory).await;
}
