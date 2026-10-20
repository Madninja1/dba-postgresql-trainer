use dba_trainer_application::{PracticeEngine, PracticeError};
use dba_trainer_domain::{
    PracticeAssertion, PracticeCheckResult, PracticeColumn, PracticeComparison,
    PracticeExecutionResult, PracticeRule, PracticeSchema, PracticeTable, PracticeTableData,
    PracticeTask, PracticeValue,
};
use rusqlite::{Connection, params, types::ValueRef};

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
            .as_ref()
            .ok_or(PracticeError::TaskNotLoaded)?;
        execute_sql(connection, sql)
    }

    fn check(&self, task: &PracticeTask, sql: &str) -> Result<PracticeCheckResult, PracticeError> {
        validate_sql(sql)?;

        let mut failed_checks = validate_rules(task, sql);
        let actual_connection = create_sandbox(task)?;
        let actual = execute_sql(&actual_connection, sql)?;

        let assertions = if task.assertions.is_empty() {
            vec![PracticeAssertion::ResultEqualsSolution]
        } else {
            task.assertions.clone()
        };

        let needs_expected = assertions
            .iter()
            .any(|assertion| matches!(assertion, PracticeAssertion::ResultEqualsSolution));

        let expected = if needs_expected {
            let expected_connection = create_sandbox(task)?;
            Some(execute_sql(&expected_connection, &task.solution_sql)?)
        } else {
            None
        };

        for assertion in &assertions {
            match assertion {
                PracticeAssertion::ResultEqualsSolution => {
                    let expected = expected.as_ref().expect("expected result should exist");
                    if !compare_results(task.comparison, expected, &actual) {
                        failed_checks
                            .push(String::from("result does not match the expected result"));
                    }
                }
                PracticeAssertion::TableExists(table) => {
                    if !object_exists(&actual_connection, "table", table)? {
                        failed_checks.push(format!("table '{table}' does not exist"));
                    }
                }
                PracticeAssertion::IndexExists(index) => {
                    if !object_exists(&actual_connection, "index", index)? {
                        failed_checks.push(format!("index '{index}' does not exist"));
                    }
                }
                PracticeAssertion::ScalarEquals { sql, expected } => {
                    let result = execute_sql(&actual_connection, sql)?;
                    let actual_value = result
                        .rows
                        .first()
                        .and_then(|row| row.first())
                        .map(ToString::to_string);
                    if actual_value.as_deref() != Some(expected.as_str()) {
                        failed_checks.push(format!(
                            "assertion returned {:?}, expected {expected}",
                            actual_value.as_deref().unwrap_or("<no value>")
                        ));
                    }
                }
                PracticeAssertion::RowCountEquals { sql, expected } => {
                    let result = execute_sql(&actual_connection, sql)?;
                    if result.row_count() != *expected {
                        failed_checks.push(format!(
                            "assertion returned {} rows, expected {expected}",
                            result.row_count()
                        ));
                    }
                }
            }
        }

        Ok(PracticeCheckResult {
            is_correct: failed_checks.is_empty(),
            actual_rows: actual.row_count(),
            expected_rows: expected
                .as_ref()
                .map_or(0, PracticeExecutionResult::row_count),
            failed_checks,
        })
    }

    fn schema(&self) -> Result<PracticeSchema, PracticeError> {
        let connection = self
            .connection
            .as_ref()
            .ok_or(PracticeError::TaskNotLoaded)?;
        load_schema(connection)
    }

    fn table_data(
        &self,
        table: &str,
        limit: usize,
        offset: usize,
    ) -> Result<PracticeTableData, PracticeError> {
        let connection = self
            .connection
            .as_ref()
            .ok_or(PracticeError::TaskNotLoaded)?;
        let schema = load_schema(connection)?;
        if !schema
            .tables
            .iter()
            .any(|candidate| candidate.name == table)
        {
            return Err(PracticeError::execution(format!("unknown table: {table}")));
        }

        let quoted = quote_identifier(table);
        let sql = format!("SELECT * FROM {quoted} LIMIT {} OFFSET {offset}", limit + 1);
        let mut result = execute_sql(connection, &sql)?;
        let has_more = result.rows.len() > limit;
        result.rows.truncate(limit);

        Ok(PracticeTableData {
            table: table.to_owned(),
            result,
            offset,
            has_more,
        })
    }
}

fn create_sandbox(task: &PracticeTask) -> Result<Connection, PracticeError> {
    let connection = Connection::open_in_memory()
        .map_err(|error| PracticeError::execution(error.to_string()))?;
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|error| PracticeError::execution(error.to_string()))?;
    connection
        .execute_batch(&task.setup_sql)
        .map_err(|error| map_sql_error(&task.setup_sql, error))?;
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

fn validate_rules(task: &PracticeTask, sql: &str) -> Vec<String> {
    let tokens = sql_tokens(sql);
    let lower = sql.to_ascii_lowercase();
    let mut failures = Vec::new();

    for rule in &task.rules {
        match rule {
            PracticeRule::RequireKeyword(keyword) => {
                let keyword_lower = keyword.to_ascii_lowercase();
                if !tokens.iter().any(|token| token == &keyword_lower) {
                    failures.push(format!("required keyword is missing: {keyword}"));
                }
            }
            PracticeRule::ForbidKeyword(keyword) => {
                let keyword_lower = keyword.to_ascii_lowercase();
                if tokens.iter().any(|token| token == &keyword_lower) {
                    failures.push(format!("forbidden keyword is used: {keyword}"));
                }
            }
            PracticeRule::ForbidSelectStar => {
                let normalized = lower.split_whitespace().collect::<Vec<_>>().join(" ");
                if normalized.contains("select *") {
                    failures.push(String::from("SELECT * is not allowed for this task"));
                }
            }
        }
    }

    failures
}

fn sql_tokens(sql: &str) -> Vec<String> {
    sql.to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .collect()
}

fn execute_sql(
    connection: &Connection,
    sql: &str,
) -> Result<PracticeExecutionResult, PracticeError> {
    let mut statement = connection
        .prepare(sql)
        .map_err(|error| map_sql_error(sql, error))?;
    let column_count = statement.column_count();

    if column_count == 0 {
        let affected_rows = statement
            .execute([])
            .map_err(|error| map_sql_error(sql, error))?;
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
        .map_err(|error| map_sql_error(sql, error))?;
    let mut result_rows = Vec::new();

    while let Some(row) = rows.next().map_err(|error| map_sql_error(sql, error))? {
        let mut values = Vec::with_capacity(column_count);
        for index in 0..column_count {
            let value = row
                .get_ref(index)
                .map_err(|error| map_sql_error(sql, error))?;
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

fn map_sql_error(sql: &str, error: rusqlite::Error) -> PracticeError {
    let message = error.to_string();
    let token = message
        .split("near \"")
        .nth(1)
        .and_then(|rest| rest.split('\"').next())
        .filter(|token| !token.is_empty());

    let position = token.and_then(|token| {
        sql.find(token).map(|byte_index| {
            let prefix = &sql[..byte_index];
            let line = prefix
                .chars()
                .filter(|character| *character == '\n')
                .count()
                + 1;
            let column = prefix
                .rsplit('\n')
                .next()
                .map_or(1, |part| part.chars().count() + 1);
            (line, column)
        })
    });

    PracticeError::execution_at(
        message,
        position.map(|value| value.0),
        position.map(|value| value.1),
    )
}

fn load_schema(connection: &Connection) -> Result<PracticeSchema, PracticeError> {
    let mut statement = connection
        .prepare(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
        )
        .map_err(|error| PracticeError::execution(error.to_string()))?;
    let names = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| PracticeError::execution(error.to_string()))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| PracticeError::execution(error.to_string()))?;

    let mut tables = Vec::with_capacity(names.len());
    for name in names {
        let pragma = format!("PRAGMA table_info({})", quote_identifier(&name));
        let mut columns_statement = connection
            .prepare(&pragma)
            .map_err(|error| PracticeError::execution(error.to_string()))?;
        let columns = columns_statement
            .query_map([], |row| {
                Ok(PracticeColumn {
                    name: row.get(1)?,
                    data_type: row.get(2)?,
                    not_null: row.get::<_, i64>(3)? != 0,
                    default_value: row.get(4)?,
                    primary_key: row.get::<_, i64>(5)? != 0,
                })
            })
            .map_err(|error| PracticeError::execution(error.to_string()))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|error| PracticeError::execution(error.to_string()))?;
        tables.push(PracticeTable { name, columns });
    }

    Ok(PracticeSchema { tables })
}

fn object_exists(connection: &Connection, kind: &str, name: &str) -> Result<bool, PracticeError> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = ?1 AND name = ?2",
            params![kind, name],
            |row| row.get(0),
        )
        .map_err(|error| PracticeError::execution(error.to_string()))?;
    Ok(count > 0)
}

fn quote_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

fn value_from_ref(value: ValueRef<'_>) -> PracticeValue {
    match value {
        ValueRef::Null => PracticeValue::Null,
        ValueRef::Integer(value) => PracticeValue::Integer(value),
        ValueRef::Real(value) => PracticeValue::Real(value),
        ValueRef::Text(value) => PracticeValue::Text(String::from_utf8_lossy(value).into_owned()),
        ValueRef::Blob(value) => {
            PracticeValue::Blob(value.iter().map(|byte| format!("{byte:02x}")).collect())
        }
    }
}

fn compare_results(
    comparison: PracticeComparison,
    expected: &PracticeExecutionResult,
    actual: &PracticeExecutionResult,
) -> bool {
    if expected.affected_rows.is_some() || actual.affected_rows.is_some() {
        return expected.affected_rows == actual.affected_rows;
    }

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
    use dba_trainer_domain::{
        PracticeAssertion, PracticeComparison, PracticeDifficulty, PracticeKind, PracticeRule,
    };

    fn task() -> PracticeTask {
        PracticeTask {
            id: 1,
            key: String::from("demo"),
            category: String::from("select"),
            title: String::from("Demo"),
            description: String::from("Return id = 2"),
            kind: PracticeKind::WriteQuery,
            comparison: PracticeComparison::Unordered,
            difficulty: PracticeDifficulty::Easy,
            starter_sql: String::new(),
            setup_sql: String::from(
                "CREATE TABLE values_table (id INTEGER); INSERT INTO values_table VALUES (1), (2);",
            ),
            solution_sql: String::from("SELECT id FROM values_table WHERE id = 2;"),
            solution_explanation: None,
            hints: vec![String::from("Filter by id")],
            rules: Vec::new(),
            assertions: vec![PracticeAssertion::ResultEqualsSolution],
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
    fn checker_enforces_rules() {
        let mut task = task();
        task.rules = vec![
            PracticeRule::RequireKeyword(String::from("where")),
            PracticeRule::ForbidSelectStar,
        ];
        let engine = SqlitePracticeEngine::new();
        let result = engine
            .check(&task, "SELECT * FROM (SELECT 2 AS id);")
            .expect("check should work");
        assert!(!result.is_correct);
        assert_eq!(result.failed_checks.len(), 2);
    }

    #[test]
    fn schema_and_table_data_are_available() {
        let task = task();
        let mut engine = SqlitePracticeEngine::new();
        engine.load_task(&task).expect("sandbox should load");
        let schema = engine.schema().expect("schema should load");
        assert_eq!(schema.tables[0].name, "values_table");
        let data = engine
            .table_data("values_table", 1, 0)
            .expect("table data should load");
        assert_eq!(data.result.rows.len(), 1);
        assert!(data.has_more);
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
    }
}
