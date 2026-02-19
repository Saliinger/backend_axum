use crate::app_state::AppState;
use crate::dto::user::UserDto;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<UserDto>>, StatusCode> {
    match state.user_service.get_all().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            let status = match e {
                crate::app_error::AppError::NotFound => StatusCode::NOT_FOUND,
                crate::app_error::AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err(status)
        }
    }
}

pub async fn get_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<UserDto>, StatusCode> {
    match state.user_service.get_by_id(id).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            let status = match e {
                crate::app_error::AppError::NotFound => StatusCode::NOT_FOUND,
                crate::app_error::AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err(status)
        }
    }
}
