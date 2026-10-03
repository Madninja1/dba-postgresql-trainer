use crate::SourceId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub id: SourceId,
    pub module: String,
    pub section: String,
    pub locator: String,
}
