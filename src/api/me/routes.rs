use crate::api::me::friends;
use crate::api::me::handler;
use crate::api::me::settings;
use crate::api::me::stats;
use crate::app_state::AppState;
use axum::{Router, routing::get};

pub fn get_me_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handler::get_me)
                .patch(handler::patch_me)
                .delete(handler::delete_me),
        )
        .nest("/stats", stats::routes::get_stats_router())
        .nest("/friends", friends::routes::get_friends_router())
        .nest("/settings", settings::routes::get_settings_router())
}
