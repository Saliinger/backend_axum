use crate::app_state::AppState;
use axum::Router;

pub async fn build_app(app_state: AppState) -> Router {
    Router::new().with_state(app_state)
}
