use backend_axum::api;
use backend_axum::app_state::AppState;
use backend_axum::config::Config;
// use backend_axum::utils;

use backend_axum::domain::{
    game::service::GameService,
    me::{
        friends::service::FriendsService, service::MeService, settings::service::SettingsService,
        stats::service::StatsService,
    },
    user::service::UserService,
};
use backend_axum::{
    domain::{
        game::repository::GameRepository, me::repository::MeRepository,
        user::repository::UserRepository,
    },
    infra::{
        db::db_init,
        repositories::{
            game_repository::GameRepository as GameRepo, me_repository::MeRepository as MeRepo,
            user_repository::UserRepository as UserRepo,
        },
    },
};
use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::parse();
    let db = db_init(config.database_url.clone())
        .await
        .expect("DB connection failed");
    let redis = redis::Client::open("redis://127.0.0.1/").unwrap(); // to change with infra::redis

    let user_repository = Arc::new(UserRepo::new(db.clone())) as Arc<dyn UserRepository>;
    let user_service = Arc::new(UserService::new(user_repository));

    let game_repository = Arc::new(GameRepo::new(db.clone())) as Arc<dyn GameRepository>;
    let game_service = Arc::new(GameService::new(game_repository));

    let me_repository = Arc::new(MeRepo::new(db.clone())) as Arc<dyn MeRepository>;

    let friend_service = Arc::new(FriendsService::new(db.clone()));
    let stats_service = Arc::new(StatsService::new(db.clone()));
    let settings_service = Arc::new(SettingsService::new(db.clone()));

    let me_service = Arc::new(MeService::new(
        me_repository,
        friend_service,
        stats_service,
        settings_service,
    ));

    let app_state = AppState {
        config,
        redis: backend_axum::infra::redis::Redis::new(),
        user_service,
        game_service,
        me_service,
    };

    let app = api::build_app(app_state.clone());
    let port = app_state.config.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on {}", addr);
    println!("\nServer listening on {}\n", addr);

    axum::serve(listener, app).await.unwrap();
}
