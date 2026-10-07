#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum MobileError {
    #[error("{0}")]
    Core(String),
}

pub(crate) fn to_mobile_error(error: impl std::fmt::Display) -> MobileError {
    MobileError::Core(error.to_string())
}
