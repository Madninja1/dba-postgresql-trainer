use std::{error::Error, fmt};

use dba_trainer_domain::{
    PracticeCheckResult, PracticeExecutionResult, PracticeSchema, PracticeTableData, PracticeTask,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticeError {
    TaskNotLoaded,
    EmptySql,
    BlockedStatement(String),
    Execution {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },
}

impl PracticeError {
    pub fn execution(message: impl Into<String>) -> Self {
        Self::Execution {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    pub fn execution_at(
        message: impl Into<String>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        Self::Execution {
            message: message.into(),
            line,
            column,
        }
    }
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
            Self::Execution {
                message,
                line,
                column,
            } => match (line, column) {
                (Some(line), Some(column)) => {
                    write!(
                        formatter,
                        "practice execution failed at {line}:{column}: {message}"
                    )
                }
                (Some(line), None) => {
                    write!(
                        formatter,
                        "practice execution failed at line {line}: {message}"
                    )
                }
                _ => write!(formatter, "practice execution failed: {message}"),
            },
        }
    }
}

impl Error for PracticeError {}

/// Execution boundary for SQL practice.
///
/// TUI depends on this trait instead of SQLite directly. A PostgreSQL implementation can be
/// added later without changing task content or workbench state.
pub trait PracticeEngine {
    fn load_task(&mut self, task: &PracticeTask) -> Result<(), PracticeError>;

    fn reset(&mut self, task: &PracticeTask) -> Result<(), PracticeError>;

    fn execute(&mut self, sql: &str) -> Result<PracticeExecutionResult, PracticeError>;

    fn check(&self, task: &PracticeTask, sql: &str) -> Result<PracticeCheckResult, PracticeError>;

    fn schema(&self) -> Result<PracticeSchema, PracticeError>;

    fn table_data(
        &self,
        table: &str,
        limit: usize,
        offset: usize,
    ) -> Result<PracticeTableData, PracticeError>;
}
