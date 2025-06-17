#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Details: {0}")]
    InternalServerError(String),
}

impl ApiError {
    pub fn new(message: String) -> Self {
        ApiError::InternalServerError(message)
    }
}

pub type Result<T> = core::result::Result<T, ApiError>;
