use crate::app_state::AppState;
use crate::dto::{auth::RegisterDto, user::UserDto};
use axum::{Json, extract::State, http::Error};

pub async fn register(
    State(_state): State<AppState>,
    Json(_to_register): Json<RegisterDto>,
) -> Result<Json<UserDto>, Error> {
    unimplemented!("todo")
}
