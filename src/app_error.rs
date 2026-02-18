use sea_orm::DbErr;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    DatabaseError(DbErr),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
