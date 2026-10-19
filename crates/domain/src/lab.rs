#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lab {
    pub id: i64,
    pub course_code: String,
    pub key: String,
    pub lab_number: i64,
    pub title: String,
    pub description: Option<String>,
    pub body: String,
}
