use axum::{Router, routing::get};

pub fn get_game_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::get_all))
        .route("/{id}", get(handler::get_by_id))
}
