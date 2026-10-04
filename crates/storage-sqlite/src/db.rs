use std::path::Path;

use rusqlite::Connection;

use crate::StorageError;

const CURRENT_SCHEMA_VERSION: i64 = 1;

const MIGRATION_0001: &str = include_str!("../migrations/0001_init.sql");

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

    let version: i64 = connection.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

    match version {
        0 => migrate_to_v1(&mut connection)?,
        CURRENT_SCHEMA_VERSION => {}
        other => {
            return Err(StorageError::UnsupportedSchemaVersion(other));
        }
    }

    Ok(connection)
}

fn migrate_to_v1(connection: &mut Connection) -> Result<(), StorageError> {
    let transaction = connection.transaction()?;

    transaction.execute_batch(MIGRATION_0001)?;

    transaction.execute_batch("PRAGMA user_version = 1")?;

    transaction.commit()?;

    Ok(())
}
