use std::{error::Error, fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum ContentError {
    Io {
        path: PathBuf,
        source: io::Error,
    },

    Json {
        document: &'static str,
        source: serde_json::Error,
    },

    Validation(Vec<String>),
}

impl fmt::Display for ContentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(formatter, "failed to read {}: {}", path.display(), source)
            }

            Self::Json { document, source } => {
                write!(formatter, "invalid JSON in {document}: {source}")
            }

            Self::Validation(errors) => {
                write!(
                    formatter,
                    "content validation failed: {}",
                    errors.join("; ")
                )
            }
        }
    }
}

impl Error for ContentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),

            Self::Json { source, .. } => Some(source),

            Self::Validation(_) => None,
        }
    }
}
