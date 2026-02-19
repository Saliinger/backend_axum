use crate::app_state::AppState;
use crate::dto::{me::PatchMeDto, user::UserDto};

use axum::{Json, extract::State, http::StatusCode};

pub async fn get_me(State(state): State<AppState>) -> Result<Json<UserDto>, StatusCode> {
    // Hardcoded user ID for now until auth is implemented
    let user_id = String::from("1");

    match state.me_service.get_me(user_id).await {
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

pub async fn patch_me(
    State(state): State<AppState>,
    Json(user): Json<PatchMeDto>,
) -> Result<Json<UserDto>, StatusCode> {
    // Hardcoded user ID for now until auth is implemented
    let user_id = String::from("1");

    match state.me_service.patch_me(user_id, user).await {
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

pub async fn delete_me(State(state): State<AppState>) -> Result<Json<UserDto>, StatusCode> {
    // Hardcoded user ID for now until auth is implemented
    let user_id = String::from("1");

    match state.me_service.delete_me(user_id).await {
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
