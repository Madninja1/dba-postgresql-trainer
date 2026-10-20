use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeKind {
    WriteQuery,
    FixQuery,
    FindLogicalError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeComparison {
    Ordered,
    Unordered,
    Scalar,
    RowCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeDifficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for PracticeDifficulty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Easy => write!(formatter, "easy"),
            Self::Medium => write!(formatter, "medium"),
            Self::Hard => write!(formatter, "hard"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticeRule {
    RequireKeyword(String),
    ForbidKeyword(String),
    ForbidSelectStar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticeAssertion {
    ResultEqualsSolution,
    TableExists(String),
    IndexExists(String),
    ScalarEquals { sql: String, expected: String },
    RowCountEquals { sql: String, expected: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeTask {
    pub id: i64,
    pub key: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub kind: PracticeKind,
    pub comparison: PracticeComparison,
    pub difficulty: PracticeDifficulty,
    pub starter_sql: String,
    pub setup_sql: String,
    pub solution_sql: String,
    pub solution_explanation: Option<String>,
    pub hints: Vec<String>,
    pub rules: Vec<PracticeRule>,
    pub assertions: Vec<PracticeAssertion>,
    pub sort_order: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PracticeValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(String),
}

impl fmt::Display for PracticeValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(formatter, "NULL"),
            Self::Integer(value) => write!(formatter, "{value}"),
            Self::Real(value) => write!(formatter, "{value}"),
            Self::Text(value) | Self::Blob(value) => write!(formatter, "{value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PracticeExecutionResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PracticeValue>>,
    pub affected_rows: Option<usize>,
}

impl PracticeExecutionResult {
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeCheckResult {
    pub is_correct: bool,
    pub actual_rows: usize,
    pub expected_rows: usize,
    pub failed_checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeColumn {
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub primary_key: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeTable {
    pub name: String,
    pub columns: Vec<PracticeColumn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PracticeSchema {
    pub tables: Vec<PracticeTable>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PracticeTableData {
    pub table: String,
    pub result: PracticeExecutionResult,
    pub offset: usize,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeTaskProgress {
    pub task_id: i64,
    pub last_sql: String,
    pub attempts: usize,
    pub solved: bool,
    pub solved_attempt: Option<usize>,
    pub updated_at: String,
}

impl PracticeTaskProgress {
    pub fn is_started(&self) -> bool {
        self.attempts > 0 || !self.last_sql.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeCategoryStats {
    pub category: String,
    pub total_tasks: usize,
    pub started_tasks: usize,
    pub solved_tasks: usize,
    pub total_attempts: usize,
    pub solved_first_try: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PracticeStats {
    pub total_tasks: usize,
    pub started_tasks: usize,
    pub solved_tasks: usize,
    pub total_attempts: usize,
    pub solved_first_try: usize,
    pub categories: Vec<PracticeCategoryStats>,
}

impl PracticeStats {
    pub fn solved_percent(&self) -> f64 {
        if self.total_tasks == 0 {
            0.0
        } else {
            self.solved_tasks as f64 * 100.0 / self.total_tasks as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_execution_reports_row_count() {
        let result = PracticeExecutionResult {
            columns: vec![String::from("id")],
            rows: vec![
                vec![PracticeValue::Integer(1)],
                vec![PracticeValue::Integer(2)],
            ],
            affected_rows: None,
        };

        assert_eq!(result.row_count(), 2);
    }

    #[test]
    fn practice_stats_reports_solved_percent() {
        let stats = PracticeStats {
            total_tasks: 4,
            solved_tasks: 3,
            ..PracticeStats::default()
        };

        assert_eq!(stats.solved_percent(), 75.0);
    }
}
