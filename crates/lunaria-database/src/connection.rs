use crate::error::DatabaseError;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait, Schema,
    Statement,
};
use std::path::Path;
use std::time::Duration;
use tempfile::TempPath;

pub(crate) async fn connect(path: &Path) -> Result<DatabaseConnection, DatabaseError> {
    let path = path.to_path_buf();
    let mut options = ConnectOptions::new("sqlite:");
    options
        .max_connections(1)
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(move |options| {
            options
                .filename(&path)
                .in_memory(false)
                .shared_cache(false)
                .create_if_missing(false)
                .foreign_keys(true)
                .busy_timeout(Duration::from_secs(5))
        });

    Ok(Database::connect(options).await?)
}

pub(crate) async fn create_table<C, E>(database: &C, entity: E) -> Result<(), DatabaseError>
where
    C: ConnectionTrait,
    E: EntityTrait,
{
    let schema = Schema::new(database.get_database_backend());
    let mut table = schema.create_table_from_entity(entity);
    database.execute(table.if_not_exists()).await?;

    for mut index in schema.create_index_from_entity(entity) {
        database.execute(index.if_not_exists()).await?;
    }

    Ok(())
}

pub(crate) async fn set_database_metadata<C>(
    database: &C,
    app_id: i64,
    schema_version: i64,
) -> Result<(), DatabaseError>
where
    C: ConnectionTrait,
{
    database
        .execute_unprepared(&format!("PRAGMA application_id = {app_id}"))
        .await?;

    database
        .execute_unprepared(&format!("PRAGMA user_version = {schema_version}"))
        .await?;

    Ok(())
}

pub(crate) async fn validate_database_metadata<C>(
    database: &C,
    path: &Path,
    expected_id: i64,
    expected_version: i64,
) -> Result<(), DatabaseError>
where
    C: ConnectionTrait,
{
    if read_integer(database, "PRAGMA application_id").await? != expected_id {
        return Err(DatabaseError::InvalidDatabase(path.to_path_buf()));
    }

    let version = read_integer(database, "PRAGMA user_version").await?;
    if version != expected_version {
        return Err(DatabaseError::UnsupportedSchemaVersion {
            path: path.to_path_buf(),
            version,
        });
    }

    Ok(())
}

pub(crate) async fn check_integrity<C>(database: &C) -> Result<(), DatabaseError>
where
    C: ConnectionTrait,
{
    let rows = database
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA integrity_check",
        ))
        .await?;

    if rows.len() != 1 || rows[0].try_get_by_index::<String>(0)? != "ok" {
        return Err(DatabaseError::IntegrityCheckFailed(
            "PRAGMA integrity_check did not return ok".to_owned(),
        ));
    }

    let violations = database
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA foreign_key_check",
        ))
        .await?;
    if !violations.is_empty() {
        return Err(DatabaseError::IntegrityCheckFailed(format!(
            "{} foreign-key violation(s)",
            violations.len()
        )));
    }

    Ok(())
}

pub(crate) fn publish(temporary: TempPath, destination: &Path) -> Result<(), DatabaseError> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&temporary)
        .and_then(|file| file.sync_all())
        .map_err(|source| DatabaseError::io(&temporary, source))?;

    temporary.persist_noclobber(destination).map_err(|error| {
        if error.error.kind() == std::io::ErrorKind::AlreadyExists {
            DatabaseError::AlreadyExists(destination.to_path_buf())
        } else {
            DatabaseError::io(destination, error.error)
        }
    })?;

    Ok(())
}

async fn read_integer<C>(database: &C, sql: &str) -> Result<i64, DatabaseError>
where
    C: ConnectionTrait,
{
    let row = database
        .query_one_raw(Statement::from_string(DbBackend::Sqlite, sql))
        .await?
        .ok_or_else(|| DatabaseError::IntegrityCheckFailed(sql.to_owned()))?;
    Ok(row.try_get_by_index(0)?)
}
