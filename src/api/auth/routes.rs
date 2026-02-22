use crate::app_state::AppState;
use axum::{Router, routing::post};
use super::handler::register;

pub fn get_routes() -> Router<AppState> {
    Router::new().route("/register", post(register))
}
