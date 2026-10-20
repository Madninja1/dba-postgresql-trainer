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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeTask {
    pub id: i64,
    pub key: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub kind: PracticeKind,
    pub comparison: PracticeComparison,
    pub starter_sql: String,
    pub setup_sql: String,
    pub solution_sql: String,
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
}
