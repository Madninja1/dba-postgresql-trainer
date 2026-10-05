use crate::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    CourseMaterial,
    PostgreSqlDocs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub id: SourceId,
    pub kind: SourceKind,

    pub module: String,
    pub section: String,
    pub locator: String,

    pub url: Option<String>,
}
