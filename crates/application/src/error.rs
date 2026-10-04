#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    NotFound,
    InvalidState(String),
    Storage(String),
}
