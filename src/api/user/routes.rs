use super::handler;
use crate::app_state::AppState;
use axum::{Router, routing::get};

pub fn get_user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::get_all))
        .route("/{id}", get(handler::get_by_id))
}