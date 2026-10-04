use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum StorageError {
    Sqlite(rusqlite::Error),
    UnsupportedSchemaVersion(i64),
}

impl From<rusqlite::Error> for StorageError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => {
                write!(formatter, "SQLite error: {error}")
            }

            Self::UnsupportedSchemaVersion(version) => {
                write!(formatter, "unsupported database schema version: {version}")
            }
        }
    }
}
