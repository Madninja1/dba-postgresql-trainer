use crate::TopicId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic {
    pub id: TopicId,

    pub course_code: String,

    pub slug: String,
    pub notes_part: i64,
    pub topic_number: i64,
    pub title: String,
    pub description: Option<String>,
}
