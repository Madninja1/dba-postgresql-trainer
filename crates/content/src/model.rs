use serde::Deserialize;

pub const CONTENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopicDocument {
    pub schema_version: u32,

    pub course: String,
    pub slug: String,

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
