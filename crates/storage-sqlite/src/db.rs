use std::path::Path;

use rusqlite::Connection;

use crate::StorageError;

const CURRENT_SCHEMA_VERSION: i64 = 6;

const MIGRATION_0001: &str = include_str!("../migrations/0001_init.sql");

const MIGRATION_0002: &str = include_str!("../migrations/0002_content_and_multiple_choice.sql");

const MIGRATION_0003: &str = include_str!("../migrations/0003_content_sync.sql");

const MIGRATION_0004: &str = include_str!("../migrations/0004_session_lifecycle.sql");

const MIGRATION_0005: &str = include_str!("../migrations/0005_course_quiz_scope.sql");

const MIGRATION_0006: &str = include_str!("../migrations/0006_topic_metadata.sql");

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

        version = 2;
    }

    if version < 3 {
        migrate(&mut connection, MIGRATION_0003, 3)?;

        version = 3;
    }

    if version < 4 {
        migrate(&mut connection, MIGRATION_0004, 4)?;

        version = 4;
    }

    if version < 5 {
        migrate(&mut connection, MIGRATION_0005, 5)?;

        version = 5;
    }

    if version < 6 {
        migrate(&mut connection, MIGRATION_0006, 6)?;

        version = 6;
    }

    debug_assert_eq!(version, CURRENT_SCHEMA_VERSION,);

    Ok(connection)
}

fn migrate(connection: &mut Connection, sql: &str, version: i64) -> Result<(), StorageError> {
    let transaction = connection.transaction()?;

    transaction.execute_batch(sql)?;

    transaction.execute_batch(&format!("PRAGMA user_version = {version};"))?;

    transaction.commit()?;

    Ok(())
}
