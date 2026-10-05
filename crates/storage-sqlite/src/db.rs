use std::path::Path;

use rusqlite::Connection;

use crate::StorageError;

const CURRENT_SCHEMA_VERSION: i64 = 2;

const MIGRATION_0001: &str = include_str!("../migrations/0001_init.sql");

const MIGRATION_0002: &str = include_str!("../migrations/0002_content_and_multiple_choice.sql");

pub(crate) fn open(path: impl AsRef<Path>) -> Result<Connection, StorageError> {
    let connection = Connection::open(path)?;

    prepare_connection(connection)
}

pub(crate) fn open_in_memory() -> Result<Connection, StorageError> {
    let connection = Connection::open_in_memory()?;

    prepare_connection(connection)
}

fn prepare_connection(mut connection: Connection) -> Result<Connection, StorageError> {
    connection.execute_batch("PRAGMA foreign_keys = ON;")?;

    let mut version: i64 = connection.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

    if version > CURRENT_SCHEMA_VERSION {
        return Err(StorageError::UnsupportedSchemaVersion(version));
    }

    if version < 1 {
        migrate(&mut connection, MIGRATION_0001, 1)?;

        version = 1;
    }

    if version < 2 {
        migrate(&mut connection, MIGRATION_0002, 2)?;
    }

    Ok(connection)
}

fn migrate(connection: &mut Connection, sql: &str, version: i64) -> Result<(), StorageError> {
    let transaction = connection.transaction()?;

    transaction.execute_batch(sql)?;

    transaction.execute_batch(&format!("PRAGMA user_version = {version};"))?;

    transaction.commit()?;

    Ok(())
}
