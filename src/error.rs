use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Details: {0}")]
    InternalServerError(String),
    #[error("Authentication Failed. {0}")]
    Unauthorized(String),
}

impl ApiError {
    pub(crate) fn new(message: String) -> Self {
        ApiError::InternalServerError(message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let (status, error_message, error_code) = match self {
            ApiError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg, "INTERNAL_ERROR")
            },
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg, "UNAUTHORIZED"),
        };

        let body = Json(json!(
            {"error": {
                "code": error_code,
                "message": error_message,
                "success": false
            }}
        ));

        (status, body).into_response()
    }
}

pub type Result<T> = core::result::Result<T, ApiError>;
