use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Details: {0}")]
    InternalServerError(String),
    #[error("Login Failed")]
    LoginFail,
}

impl ApiError {
    pub(crate) fn new(message: String) -> Self {
        ApiError::InternalServerError(message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

pub type Result<T> = core::result::Result<T, ApiError>;
