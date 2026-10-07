use crate::DatabaseError;
use crate::connection::{
    check_integrity, connect, create_table, publish, read_integer, set_database_metadata,
    validate_database_metadata,
};
use crate::project::recent_project;
use crate::settings::app_setting;
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

async fn prepare_schema(database: &DatabaseConnection, path: &Path) -> Result<(), DatabaseError> {
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
            return Err(DatabaseError::UnsupportedSchemaVersion {
                path: path.to_path_buf(),
                version,
            });
        }
    }

    // Validation is part of the same transaction as DDL and the version change.
    // A malformed legacy schema rolls back both instead of publishing v2 metadata.
    validate(&transaction, path).await?;
    transaction.commit().await?;
    Ok(())
}

async fn validate<C>(database: &C, path: &Path) -> Result<(), DatabaseError>
where
    C: ConnectionTrait,
{
    validate_database_metadata(database, path, APP_DATABASE_ID, APP_SCHEMA_VERSION).await?;

    recent_project::Entity::find()
        .limit(0)
        .all(database)
        .await
        .map_err(|_| DatabaseError::InvalidDatabase(path.to_path_buf()))?;

    app_setting::Entity::find()
        .limit(0)
        .all(database)
        .await
        .map_err(|_| DatabaseError::InvalidDatabase(path.to_path_buf()))?;

    check_integrity(database).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseManager;
    use crate::settings::repository::shortcuts::Repository;
    use lunaria_core::settings::shortcuts::model::{ShortcutAction, ShortcutOverrides};
    use lunaria_core::settings::shortcuts::repository::ShortcutRepository;
    use std::sync::Arc;
    use tempfile::{TempDir, tempdir};

    const SAVED_SHORTCUTS: &str = r#"{"open_settings":null}"#;

    async fn legacy_database(directory: &TempDir, version: i64) -> DatabaseConnection {
        let path = directory.path().join(APP_DATABASE_FILE);
        tokio::fs::write(&path, []).await.unwrap();
        let database = connect(&path).await.unwrap();
        database
            .execute_unprepared(
                "CREATE TABLE recent_project (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    project_name TEXT NOT NULL,
                    project_path TEXT NOT NULL UNIQUE,
                    created_at INTEGER NOT NULL,
                    last_opened_at INTEGER NOT NULL
                )",
            )
            .await
            .unwrap();
        database
            .execute_unprepared(
                "INSERT INTO recent_project VALUES (7, 'Existing project', '/saved/project', 11, 22)",
            )
            .await
            .unwrap();
        set_database_metadata(&database, APP_DATABASE_ID, version)
            .await
            .unwrap();
        database
    }

    async fn add_legacy_settings(database: &DatabaseConnection) {
        database
            .execute_unprepared(
                "CREATE TABLE app_setting (key TEXT NOT NULL PRIMARY KEY, value TEXT NOT NULL)",
            )
            .await
            .unwrap();
        for (key, value) in [("shortcuts", SAVED_SHORTCUTS), ("theme", "system")] {
            app_setting::Entity::insert(app_setting::ActiveModel {
                key: sea_orm::Set(key.to_owned()),
                value: sea_orm::Set(value.to_owned()),
            })
            .exec(database)
            .await
            .unwrap();
        }
    }

    async fn assert_recent_preserved(database: &DatabaseConnection) {
        let record = recent_project::Entity::find_by_id(7)
            .one(database)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(record.project_name, "Existing project");
        assert_eq!(record.project_path, "/saved/project");
        assert_eq!(record.created_at, 11);
        assert_eq!(record.last_opened_at, 22);
    }

    async fn table_exists(database: &DatabaseConnection, table: &str) -> bool {
        database
            .query_one_raw(sea_orm::Statement::from_sql_and_values(
                sea_orm::DbBackend::Sqlite,
                "SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?",
                [table.into()],
            ))
            .await
            .unwrap()
            .is_some()
    }

    async fn initialization_error(directory: &TempDir) -> DatabaseError {
        match AppDatabase::open_or_create(directory.path()).await {
            Err(error) => error,
            Ok(database) => {
                database.connection.close().await.unwrap();
                panic!("expected schema initialization to reject the database");
            }
        }
    }

    #[tokio::test]
    async fn new_database_has_complete_schema_before_repository_construction() {
        let directory = tempdir().unwrap();
        let databases = Arc::new(DatabaseManager::initialize(directory.path()).await.unwrap());
        let connection = databases.app_database();
        assert_eq!(
            read_integer(connection, "PRAGMA user_version")
                .await
                .unwrap(),
            2
        );
        assert!(table_exists(connection, "recent_project").await);
        assert!(table_exists(connection, "app_setting").await);

        let repository = Repository::new(databases.clone());
        assert!(repository.load().await.unwrap().is_empty());
        let overrides = ShortcutOverrides::from([(ShortcutAction::OpenSettings, None)]);
        repository.save(&overrides).await.unwrap();
        assert_eq!(repository.load().await.unwrap(), overrides);
        connection.clone().close().await.unwrap();
    }

    #[tokio::test]
    async fn version_one_without_settings_upgrades_and_reopens_without_data_loss() {
        let directory = tempdir().unwrap();
        legacy_database(&directory, 1).await.close().await.unwrap();

        for _ in 0..2 {
            let database = AppDatabase::open_or_create(directory.path()).await.unwrap();
            assert_eq!(
                read_integer(database.connection(), "PRAGMA user_version")
                    .await
                    .unwrap(),
                2
            );
            assert_recent_preserved(database.connection()).await;
            assert!(
                app_setting::Entity::find()
                    .all(database.connection())
                    .await
                    .unwrap()
                    .is_empty()
            );
            database.connection.close().await.unwrap();
        }
    }

    #[tokio::test]
    async fn version_one_existing_settings_and_recent_projects_are_preserved() {
        let directory = tempdir().unwrap();
        let legacy = legacy_database(&directory, 1).await;
        add_legacy_settings(&legacy).await;
        legacy.close().await.unwrap();

        let databases = Arc::new(DatabaseManager::initialize(directory.path()).await.unwrap());
        let connection = databases.app_database();
        assert_recent_preserved(connection).await;
        assert_eq!(
            read_integer(connection, "PRAGMA user_version")
                .await
                .unwrap(),
            2
        );
        for (key, value) in [("shortcuts", SAVED_SHORTCUTS), ("theme", "system")] {
            assert_eq!(
                app_setting::Entity::find_by_id(key)
                    .one(connection)
                    .await
                    .unwrap()
                    .unwrap()
                    .value,
                value,
            );
        }
        assert_eq!(
            Repository::new(databases.clone()).load().await.unwrap(),
            ShortcutOverrides::from([(ShortcutAction::OpenSettings, None)]),
        );
        connection.clone().close().await.unwrap();
    }

    #[tokio::test]
    async fn unknown_versions_and_foreign_application_ids_are_rejected_without_changes() {
        for (version, app_id) in [(0, APP_DATABASE_ID), (3, APP_DATABASE_ID), (1, 123)] {
            let directory = tempdir().unwrap();
            let legacy = legacy_database(&directory, version).await;
            set_database_metadata(&legacy, app_id, version)
                .await
                .unwrap();
            legacy.close().await.unwrap();

            let error = initialization_error(&directory).await;
            if app_id != APP_DATABASE_ID {
                assert!(matches!(error, DatabaseError::InvalidDatabase(_)));
            } else {
                assert!(
                    matches!(error, DatabaseError::UnsupportedSchemaVersion { version: actual, .. } if actual == version)
                );
            }

            let unchanged = connect(&directory.path().join(APP_DATABASE_FILE))
                .await
                .unwrap();
            assert_eq!(
                read_integer(&unchanged, "PRAGMA user_version")
                    .await
                    .unwrap(),
                version
            );
            assert_eq!(
                read_integer(&unchanged, "PRAGMA application_id")
                    .await
                    .unwrap(),
                app_id
            );
            assert!(!table_exists(&unchanged, "app_setting").await);
            assert_recent_preserved(&unchanged).await;
            unchanged.close().await.unwrap();
        }
    }

    #[tokio::test]
    async fn malformed_legacy_schema_rolls_back_new_table_and_version_change() {
        let directory = tempdir().unwrap();
        let legacy = legacy_database(&directory, 1).await;
        legacy
            .execute_unprepared(
                "ALTER TABLE recent_project RENAME COLUMN project_path TO legacy_path",
            )
            .await
            .unwrap();
        legacy.close().await.unwrap();

        assert!(matches!(
            initialization_error(&directory).await,
            DatabaseError::InvalidDatabase(_)
        ));
        let unchanged = connect(&directory.path().join(APP_DATABASE_FILE))
            .await
            .unwrap();
        assert_eq!(
            read_integer(&unchanged, "PRAGMA user_version")
                .await
                .unwrap(),
            1
        );
        assert!(!table_exists(&unchanged, "app_setting").await);
        let record = unchanged
            .query_one_raw(sea_orm::Statement::from_string(
                sea_orm::DbBackend::Sqlite,
                "SELECT legacy_path FROM recent_project WHERE id = 7",
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            record.try_get_by_index::<String>(0).unwrap(),
            "/saved/project"
        );
        unchanged.close().await.unwrap();
    }

    #[tokio::test]
    async fn malformed_existing_settings_are_rejected_without_replacing_data() {
        let directory = tempdir().unwrap();
        let legacy = legacy_database(&directory, 1).await;
        legacy
            .execute_unprepared(
                "CREATE TABLE app_setting (key TEXT PRIMARY KEY, legacy_value TEXT)",
            )
            .await
            .unwrap();
        legacy
            .execute_unprepared("INSERT INTO app_setting VALUES ('theme', 'keep-me')")
            .await
            .unwrap();
        legacy.close().await.unwrap();

        assert!(matches!(
            initialization_error(&directory).await,
            DatabaseError::InvalidDatabase(_)
        ));
        let unchanged = connect(&directory.path().join(APP_DATABASE_FILE))
            .await
            .unwrap();
        assert_eq!(
            read_integer(&unchanged, "PRAGMA user_version")
                .await
                .unwrap(),
            1
        );
        let record = unchanged
            .query_one_raw(sea_orm::Statement::from_string(
                sea_orm::DbBackend::Sqlite,
                "SELECT legacy_value FROM app_setting WHERE key = 'theme'",
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(record.try_get_by_index::<String>(0).unwrap(), "keep-me");
        assert_recent_preserved(&unchanged).await;
        unchanged.close().await.unwrap();
    }

    #[tokio::test]
    async fn current_schema_with_missing_settings_table_is_rejected_without_repairing_it() {
        let directory = tempdir().unwrap();
        legacy_database(&directory, 2).await.close().await.unwrap();

        assert!(matches!(
            initialization_error(&directory).await,
            DatabaseError::InvalidDatabase(_)
        ));
        let unchanged = connect(&directory.path().join(APP_DATABASE_FILE))
            .await
            .unwrap();
        assert_eq!(
            read_integer(&unchanged, "PRAGMA user_version")
                .await
                .unwrap(),
            2
        );
        assert!(!table_exists(&unchanged, "app_setting").await);
        assert_recent_preserved(&unchanged).await;
        unchanged.close().await.unwrap();
    }
}
