use crate::TopicId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    pub id: TopicId,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
}
