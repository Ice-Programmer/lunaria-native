use anyhow::{Context, Result, bail};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait, Schema,
    Statement,
};
use std::path::Path;
use std::time::Duration;
use tempfile::TempPath;

pub(crate) async fn connect(path: &Path) -> Result<DatabaseConnection> {
    let sqlite_path = path.to_path_buf();
    let mut options = ConnectOptions::new("sqlite:");
    options
        .max_connections(1)
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(move |options| {
            options
                .filename(&sqlite_path)
                .in_memory(false)
                .shared_cache(false)
                .create_if_missing(false)
                .foreign_keys(true)
                .busy_timeout(Duration::from_secs(5))
        });

    Database::connect(options)
        .await
        .with_context(|| format!("failed to connect to database: {}", path.display()))
}

pub(crate) async fn create_table<C, E>(database: &C, entity: E) -> Result<()>
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
) -> Result<()>
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
) -> Result<()>
where
    C: ConnectionTrait,
{
    if read_integer(database, "PRAGMA application_id").await? != expected_id {
        bail!("invalid Lunaria database: {}", path.display());
    }

    let version = read_integer(database, "PRAGMA user_version").await?;
    if version != expected_version {
        bail!(
            "unsupported database schema version {version}: {}",
            path.display()
        );
    }

    Ok(())
}

pub(crate) async fn check_integrity<C>(database: &C) -> Result<()>
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
        bail!("database integrity check failed: PRAGMA integrity_check did not return ok");
    }

    let violations = database
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA foreign_key_check",
        ))
        .await?;
    if !violations.is_empty() {
        bail!(
            "database integrity check failed: {} foreign-key violation(s)",
            violations.len()
        );
    }

    Ok(())
}

pub(crate) fn publish(temporary: TempPath, destination: &Path) -> Result<()> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&temporary)
        .and_then(|file| file.sync_all())
        .with_context(|| format!("failed to sync temporary database: {}", temporary.display()))?;

    temporary.persist_noclobber(destination).map_err(|error| {
        let message = if error.error.kind() == std::io::ErrorKind::AlreadyExists {
            format!("file already exists: {}", destination.display())
        } else {
            format!("failed to publish database: {}", destination.display())
        };
        anyhow::Error::new(error.error).context(message)
    })?;

    Ok(())
}

pub(crate) async fn read_integer<C>(database: &C, sql: &str) -> Result<i64>
where
    C: ConnectionTrait,
{
    let row = database
        .query_one_raw(Statement::from_string(DbBackend::Sqlite, sql))
        .await?
        .with_context(|| format!("database integrity check failed: {sql}"))?;
    Ok(row.try_get_by_index(0)?)
}
