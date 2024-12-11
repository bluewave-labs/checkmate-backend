use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    ServerError,
    Unauthorized(String),
    FormatError(String),
    DatabaseError,
    Conflict(String),
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Fount".to_string()),
            AppError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Not Fount".to_string()),
            AppError::DatabaseError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Database Error".to_string(),
            ),
            AppError::Conflict(field) => {
                (StatusCode::CONFLICT, format!("{} already exists", field))
            }
            AppError::ValidationError(msg) => (StatusCode::NOT_FOUND, format!("{}", msg)),
            AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            AppError::FormatError(field) => {
                (StatusCode::UNPROCESSABLE_ENTITY, format!("{}", field))
            }
        };
        (status, Json(json!({"message":error_code}))).into_response()
    }
}
