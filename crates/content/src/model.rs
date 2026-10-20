use serde::Deserialize;

pub const CONTENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopicDocument {
    pub schema_version: u32,

    pub course: String,
    pub slug: String,

    pub notes_part: i64,
    pub topic_number: i64,

    pub title: String,
    pub description: Option<String>,

    pub sort_order: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionsDocument {
    pub schema_version: u32,

    pub topic: String,

    pub questions: Vec<QuestionDocument>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionDocument {
    pub key: String,

    #[serde(rename = "type")]
    pub question_type: QuestionTypeDocument,

    pub text: String,

    pub answers: Vec<AnswerDocument>,

    pub explanation: String,

    pub source: SourceDocument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionTypeDocument {
    SingleChoice,
    MultipleChoice,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerDocument {
    pub key: String,
    pub text: String,
    pub correct: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SourceKindDocument {
    #[serde(rename = "course_material")]
    CourseMaterial,

    #[serde(rename = "postgresql_docs")]
    PostgreSqlDocs,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceDocument {
    pub key: String,

    pub kind: SourceKindDocument,

    pub module: String,
    pub section: String,
    pub locator: String,

    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentBundle {
    pub topic: TopicDocument,
    pub questions: QuestionsDocument,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabDocument {
    pub schema_version: u32,
    pub course: String,
    pub key: String,
    pub lab_number: i64,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabBundle {
    pub lab: LabDocument,
    pub body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeKindDocument {
    WriteQuery,
    FixQuery,
    FindLogicalError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeComparisonDocument {
    Ordered,
    Unordered,
    Scalar,
    RowCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PracticeDifficultyDocument {
    Easy,
    #[default]
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PracticeRuleDocument {
    RequireKeyword { keyword: String },
    ForbidKeyword { keyword: String },
    ForbidSelectStar,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PracticeAssertionDocument {
    ResultEqualsSolution,
    TableExists { table: String },
    IndexExists { index: String },
    ScalarEquals { sql: String, expected: String },
    RowCountEquals { sql: String, expected: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeDocument {
    pub schema_version: u32,
    pub key: String,
    pub category: String,
    pub title: String,
    pub description: String,

    #[serde(rename = "type")]
    pub practice_type: PracticeKindDocument,

    pub comparison: PracticeComparisonDocument,

    #[serde(default)]
    pub difficulty: PracticeDifficultyDocument,

    #[serde(default)]
    pub starter_sql: String,

    #[serde(default)]
    pub hints: Vec<String>,

    #[serde(default)]
    pub solution_explanation: Option<String>,

    #[serde(default)]
    pub rules: Vec<PracticeRuleDocument>,

    #[serde(default)]
    pub assertions: Vec<PracticeAssertionDocument>,

    pub sort_order: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeBundle {
    pub practice: PracticeDocument,
    pub setup_sql: String,
    pub solution_sql: String,
}
