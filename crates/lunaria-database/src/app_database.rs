use crate::DatabaseError;
use crate::connection::{
    check_integrity, connect, create_table, publish, set_database_metadata,
    validate_database_metadata,
};
use crate::project::recent_project;
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect, TransactionTrait};
use std::path::Path;
use tempfile::NamedTempFile;

const APP_DATABASE_FILE: &str = "app.db";
const APP_DATABASE_ID: i64 = 0x4c554e42;
const APP_SCHEMA_VERSION: i64 = 1;

pub(crate) struct AppDatabase {
    connection: DatabaseConnection,
}

impl AppDatabase {
    pub(crate) async fn open_or_create(directory: &Path) -> Result<Self, DatabaseError> {
        tokio::fs::create_dir_all(directory)
            .await
            .map_err(|source| DatabaseError::IO {
                path: directory.to_path_buf(),
                source,
            })?;

        let path = directory.join(APP_DATABASE_FILE);
        if !tokio::fs::try_exists(&path)
            .await
            .map_err(|source| DatabaseError::IO {
                path: path.to_path_buf(),
                source,
            })?
        {
            create_database_file(directory, &path).await?;
        }

        let connection = connect(&path).await?;
        if let Err(error) = validate(&connection, &path).await {
            connection.close().await?;
            return Err(error);
        }

        Ok(Self { connection })
    }

    pub(crate) fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

async fn create_database_file(directory: &Path, destination: &Path) -> Result<(), DatabaseError> {
    let temporary = NamedTempFile::new_in(directory)
        .map_err(|source| DatabaseError::IO {
            path: directory.to_path_buf(),
            source,
        })?
        .into_temp_path();

    let database = connect(&temporary).await?;

    let result: Result<(), DatabaseError> = async {
        let transaction = database.begin().await?;
        create_table(&transaction, recent_project::Entity).await?;
        set_database_metadata(&transaction, APP_DATABASE_ID, APP_SCHEMA_VERSION).await?;
        transaction.commit().await?;
        Ok(())
    }
    .await;

    database.close().await?;
    result?;
    publish(temporary, destination)
}

async fn validate(database: &DatabaseConnection, path: &Path) -> Result<(), DatabaseError> {
    validate_database_metadata(database, path, APP_DATABASE_ID, APP_SCHEMA_VERSION).await?;

    recent_project::Entity::find()
        .limit(0)
        .all(database)
        .await
        .map_err(|_| DatabaseError::InvalidDatabase(path.to_path_buf()))?;

    check_integrity(database).await
}
