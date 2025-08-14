use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Conflict(&'static str),
    #[error("{0}")]
    NotFound(&'static str),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}
