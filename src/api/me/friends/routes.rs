use crate::app_state::AppState;
use axum::Router;

pub fn get_friends_router() -> Router<AppState> {
    Router::new()
}
