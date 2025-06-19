use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Details: {0}")]
    InternalServerError(String),
    #[error("Authentication Failed. {0}")]
    Unauthorized(String),
    #[error("Delete operation failed, Ticket wit id: {id} not found.")]
    DeleteFailedIdNotFound { id: String },
}

impl ApiError {
    pub(crate) fn new(message: String) -> Self {
        ApiError::InternalServerError(message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let status = match self {
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::DeleteFailedIdNotFound { .. } => StatusCode::NOT_FOUND,
        };

        let body = Json(json!(
            {"error": {
                "code": status.to_string(),
                "message": self.to_string(),
                "success": false
            }}
        ));

        (status, body).into_response()
    }
}

pub type Result<T> = core::result::Result<T, ApiError>;
