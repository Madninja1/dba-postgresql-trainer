use dba_trainer_application::{PracticeEngine, PracticeError};
use dba_trainer_domain::{
    PracticeCheckResult, PracticeComparison, PracticeExecutionResult, PracticeTask, PracticeValue,
};
use rusqlite::{Connection, types::ValueRef};

pub struct SqlitePracticeEngine {
    connection: Option<Connection>,
}

impl SqlitePracticeEngine {
    pub fn new() -> Self {
        Self { connection: None }
    }
}

impl Default for SqlitePracticeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PracticeEngine for SqlitePracticeEngine {
    fn load_task(&mut self, task: &PracticeTask) -> Result<(), PracticeError> {
        self.connection = Some(create_sandbox(task)?);
        Ok(())
    }

    fn reset(&mut self, task: &PracticeTask) -> Result<(), PracticeError> {
        self.load_task(task)
    }

    fn execute(&mut self, sql: &str) -> Result<PracticeExecutionResult, PracticeError> {
        validate_sql(sql)?;

        let connection = self
            .connection
            .as_mut()
            .ok_or(PracticeError::TaskNotLoaded)?;

        execute_sql(connection, sql)
    }

    fn check(&self, task: &PracticeTask, sql: &str) -> Result<PracticeCheckResult, PracticeError> {
        validate_sql(sql)?;

        let mut expected_connection = create_sandbox(task)?;
        let expected = execute_sql(&mut expected_connection, &task.solution_sql)?;

        let mut actual_connection = create_sandbox(task)?;
        let actual = execute_sql(&mut actual_connection, sql)?;

        Ok(PracticeCheckResult {
            is_correct: compare_results(task.comparison, &expected, &actual),
            actual_rows: actual.row_count(),
            expected_rows: expected.row_count(),
        })
    }
}

fn create_sandbox(task: &PracticeTask) -> Result<Connection, PracticeError> {
    let connection = Connection::open_in_memory()
        .map_err(|error| PracticeError::Execution(error.to_string()))?;

    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|error| PracticeError::Execution(error.to_string()))?;

    connection
        .execute_batch(&task.setup_sql)
        .map_err(|error| PracticeError::Execution(error.to_string()))?;

    Ok(connection)
}

fn validate_sql(sql: &str) -> Result<(), PracticeError> {
    let trimmed = sql.trim();

    if trimmed.is_empty() {
        return Err(PracticeError::EmptySql);
    }

    let lowercase = trimmed.to_ascii_lowercase();
    let blocked = lowercase
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .find(|token| matches!(*token, "attach" | "detach" | "vacuum" | "load_extension"));

    if let Some(statement) = blocked {
        return Err(PracticeError::BlockedStatement(statement.to_owned()));
    }

    Ok(())
}

fn execute_sql(
    connection: &mut Connection,
    sql: &str,
) -> Result<PracticeExecutionResult, PracticeError> {
    let mut statement = connection
        .prepare(sql)
        .map_err(|error| PracticeError::Execution(error.to_string()))?;

    let column_count = statement.column_count();

    if column_count == 0 {
        let affected_rows = statement
            .execute([])
            .map_err(|error| PracticeError::Execution(error.to_string()))?;

        return Ok(PracticeExecutionResult {
            columns: Vec::new(),
            rows: Vec::new(),
            affected_rows: Some(affected_rows),
        });
    }

    let columns = statement
        .column_names()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let mut rows = statement
        .query([])
        .map_err(|error| PracticeError::Execution(error.to_string()))?;

    let mut result_rows = Vec::new();

    while let Some(row) = rows
        .next()
        .map_err(|error| PracticeError::Execution(error.to_string()))?
    {
        let mut values = Vec::with_capacity(column_count);

        for index in 0..column_count {
            let value = row
                .get_ref(index)
                .map_err(|error| PracticeError::Execution(error.to_string()))?;

            values.push(value_from_ref(value));
        }

        result_rows.push(values);
    }

    Ok(PracticeExecutionResult {
        columns,
        rows: result_rows,
        affected_rows: None,
    })
}

fn value_from_ref(value: ValueRef<'_>) -> PracticeValue {
    match value {
        ValueRef::Null => PracticeValue::Null,
        ValueRef::Integer(value) => PracticeValue::Integer(value),
        ValueRef::Real(value) => PracticeValue::Real(value),
        ValueRef::Text(value) => PracticeValue::Text(String::from_utf8_lossy(value).into_owned()),
        ValueRef::Blob(value) => {
            let text = value
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>();
            PracticeValue::Blob(text)
        }
    }
}

fn compare_results(
    comparison: PracticeComparison,
    expected: &PracticeExecutionResult,
    actual: &PracticeExecutionResult,
) -> bool {
    match comparison {
        PracticeComparison::Ordered => {
            expected.columns.len() == actual.columns.len() && expected.rows == actual.rows
        }
        PracticeComparison::Unordered => {
            expected.columns.len() == actual.columns.len()
                && unordered_rows_equal(&expected.rows, &actual.rows)
        }
        PracticeComparison::Scalar => match (
            expected.rows.first().and_then(|row| row.first()),
            actual.rows.first().and_then(|row| row.first()),
        ) {
            (Some(expected), Some(actual)) => expected == actual,
            _ => false,
        },
        PracticeComparison::RowCount => expected.row_count() == actual.row_count(),
    }
}

fn unordered_rows_equal(expected: &[Vec<PracticeValue>], actual: &[Vec<PracticeValue>]) -> bool {
    if expected.len() != actual.len() {
        return false;
    }

    let mut matched = vec![false; actual.len()];

    for expected_row in expected {
        let Some(index) = actual
            .iter()
            .enumerate()
            .find(|(index, actual_row)| !matched[*index] && *actual_row == expected_row)
            .map(|(index, _)| index)
        else {
            return false;
        };

        matched[index] = true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use dba_trainer_domain::{PracticeComparison, PracticeKind};

    fn task() -> PracticeTask {
        PracticeTask {
            id: 1,
            key: String::from("demo"),
            category: String::from("select"),
            title: String::from("Demo"),
            description: String::from("Return id = 2"),
            kind: PracticeKind::WriteQuery,
            comparison: PracticeComparison::Unordered,
            starter_sql: String::new(),
            setup_sql: String::from(
                "CREATE TABLE values_table (id INTEGER); INSERT INTO values_table VALUES (1), (2);",
            ),
            solution_sql: String::from("SELECT id FROM values_table WHERE id = 2;"),
            sort_order: 0,
        }
    }

    #[test]
    fn sandbox_keeps_changes_until_reset() {
        let task = task();
        let mut engine = SqlitePracticeEngine::new();
        engine.load_task(&task).expect("sandbox should load");

        engine
            .execute("DELETE FROM values_table WHERE id = 1;")
            .expect("delete should work");

        let result = engine
            .execute("SELECT id FROM values_table;")
            .expect("select should work");

        assert_eq!(result.rows, vec![vec![PracticeValue::Integer(2)]]);

        engine.reset(&task).expect("sandbox should reset");
        let result = engine
            .execute("SELECT id FROM values_table ORDER BY id;")
            .expect("select should work");

        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    fn checker_compares_query_results_on_clean_sandbox() {
        let task = task();
        let engine = SqlitePracticeEngine::new();

        let correct = engine
            .check(&task, "SELECT id FROM values_table WHERE id > 1;")
            .expect("check should work");
        let incorrect = engine
            .check(&task, "SELECT id FROM values_table;")
            .expect("check should work");

        assert!(correct.is_correct);
        assert!(!incorrect.is_correct);
        assert_eq!(incorrect.expected_rows, 1);
        assert_eq!(incorrect.actual_rows, 2);
    }

    #[test]
    fn sandbox_rejects_attach() {
        let task = task();
        let mut engine = SqlitePracticeEngine::new();
        engine.load_task(&task).expect("sandbox should load");

        let error = engine
            .execute("ATTACH DATABASE 'other.db' AS other;")
            .expect_err("attach should be blocked");

        assert!(matches!(error, PracticeError::BlockedStatement(_)));

        let error = engine
            .execute("VACUUM INTO 'copy.db';")
            .expect_err("vacuum into should be blocked");

        assert!(matches!(error, PracticeError::BlockedStatement(_)));
    }
}
