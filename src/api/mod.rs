use crate::app_state::AppState;
use axum::Router;
use std::sync::Arc;

pub fn build_app(app_state: Arc<AppState>) -> Router {
    Router::new().with_state(app_state)
}

// add cors header
// add auth middleware
