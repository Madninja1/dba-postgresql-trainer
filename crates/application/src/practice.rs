use std::{error::Error, fmt};

use dba_trainer_domain::{PracticeCheckResult, PracticeExecutionResult, PracticeTask};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticeError {
    TaskNotLoaded,
    EmptySql,
    BlockedStatement(String),
    Execution(String),
}

impl fmt::Display for PracticeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TaskNotLoaded => write!(formatter, "practice task is not loaded"),
            Self::EmptySql => write!(formatter, "SQL must not be empty"),
            Self::BlockedStatement(statement) => {
                write!(
                    formatter,
                    "statement is not allowed in the local sandbox: {statement}"
                )
            }
            Self::Execution(message) => write!(formatter, "practice execution failed: {message}"),
        }
    }
}

impl Error for PracticeError {}

/// Execution boundary for SQL practice.
///
/// The application and UI depend on this trait instead of a concrete database engine. The first
/// implementation is local SQLite; a PostgreSQL-backed implementation can be added later without
/// changing practice tasks or UI state.
pub trait PracticeEngine {
    fn load_task(&mut self, task: &PracticeTask) -> Result<(), PracticeError>;

    fn reset(&mut self, task: &PracticeTask) -> Result<(), PracticeError>;

    fn execute(&mut self, sql: &str) -> Result<PracticeExecutionResult, PracticeError>;

    fn check(&self, task: &PracticeTask, sql: &str) -> Result<PracticeCheckResult, PracticeError>;
}
