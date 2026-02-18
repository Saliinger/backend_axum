use crate::api::user::routes::get_user_router;
use crate::app_state::AppState;
use axum::Router;

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .nest("/user", get_user_router())
        .with_state(state)
}

// add cors header
// add auth middleware
