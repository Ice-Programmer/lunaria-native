use crate::connection::{
    check_integrity, connect, create_table, publish, read_integer, set_database_metadata,
    validate_database_metadata,
};
use crate::project::recent_project;
use crate::settings::app_setting;
use anyhow::{Context, Result, bail};
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, QuerySelect, TransactionTrait};
use std::path::Path;
use tempfile::NamedTempFile;

const APP_DATABASE_FILE: &str = "app.db";
const APP_DATABASE_ID: i64 = 0x4c554e42;
const APP_SCHEMA_VERSION: i64 = 2;

pub(crate) struct AppDatabase {
    connection: DatabaseConnection,
}

impl AppDatabase {
    pub(crate) async fn open_or_create(directory: &Path) -> Result<Self> {
        tokio::fs::create_dir_all(directory)
            .await
            .with_context(|| {
                format!(
                    "failed to create database directory: {}",
                    directory.display()
                )
            })?;

        let path = directory.join(APP_DATABASE_FILE);
        if !tokio::fs::try_exists(&path)
            .await
            .with_context(|| format!("failed to check database path: {}", path.display()))?
        {
            create_database_file(directory, &path).await?;
        }

        let connection = connect(&path).await?;
        if let Err(error) = prepare_schema(&connection, &path).await {
            connection.close().await?;
            return Err(error);
        }

        Ok(Self { connection })
    }

    pub(crate) fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

async fn create_database_file(directory: &Path, destination: &Path) -> Result<()> {
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
        create_table(&transaction, recent_project::Entity).await?;
        create_table(&transaction, app_setting::Entity).await?;
        set_database_metadata(&transaction, APP_DATABASE_ID, APP_SCHEMA_VERSION).await?;
        validate(&transaction, destination).await?;
        transaction.commit().await?;
        Ok(())
    }
    .await;

    database.close().await?;
    result?;
    publish(temporary, destination)
}

async fn prepare_schema(database: &DatabaseConnection, path: &Path) -> Result<()> {
    let transaction = database.begin().await?;
    let version = read_integer(&transaction, "PRAGMA user_version").await?;

    // Verify ownership before writing to any existing SQLite file.
    validate_database_metadata(&transaction, path, APP_DATABASE_ID, version).await?;

    match version {
        1 => {
            // v1 repositories used to create app_setting lazily. Existing rows are
            // preserved; databases that never used settings receive the table now.
            create_table(&transaction, app_setting::Entity).await?;
            set_database_metadata(&transaction, APP_DATABASE_ID, APP_SCHEMA_VERSION).await?;
        }
        APP_SCHEMA_VERSION => {}
        _ => {
            bail!(
                "unsupported database schema version {version}: {}",
                path.display()
            );
        }
    }

    // Validation is part of the same transaction as DDL and the version change.
    // A malformed legacy schema rolls back both instead of publishing v2 metadata.
    validate(&transaction, path).await?;
    transaction.commit().await?;
    Ok(())
}

async fn validate<C>(database: &C, path: &Path) -> Result<()>
where
    C: ConnectionTrait,
{
    validate_database_metadata(database, path, APP_DATABASE_ID, APP_SCHEMA_VERSION).await?;

    recent_project::Entity::find()
        .limit(0)
        .all(database)
        .await
        .with_context(|| format!("invalid Lunaria database: {}", path.display()))?;

    app_setting::Entity::find()
        .limit(0)
        .all(database)
        .await
        .with_context(|| format!("invalid Lunaria database: {}", path.display()))?;

    check_integrity(database).await
}
