use crate::app_state::AppState;
use axum::Router;

pub fn get_stats_router() -> Router<AppState> {
    Router::new()
}
