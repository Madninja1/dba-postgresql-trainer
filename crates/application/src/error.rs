use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    NotFound,
    NoQuestions,
    InvalidState(String),
    Storage(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => {
                write!(formatter, "data not found")
            }

            Self::NoQuestions => {
                write!(formatter, "no questions available")
            }

            Self::InvalidState(message) => {
                write!(formatter, "invalid state: {message}")
            }

            Self::Storage(message) => {
                write!(formatter, "storage error: {message}")
            }
        }
    }
}

impl Error for RepositoryError {}
