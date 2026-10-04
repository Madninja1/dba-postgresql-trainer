#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    NotFound,
    NoQuestions,
    InvalidState(String),
    Storage(String),
}
