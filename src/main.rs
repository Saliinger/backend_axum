use argon2::password_hash::rand_core::le;
use backend_axum::api;
use backend_axum::app_state::AppState;
use backend_axum::config::Config;
use backend_axum::infra;
// use backend_axum::utils;
use backend_axum::domain::game::service::GameService;
use backend_axum::domain::user::service::UserService;

use clap::Parser;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::parse();
    let db = infra::db::db_init(config.database_url.clone())
        .await
        .expect("DB connection failed");
    let redis = redis::Client::open("redis://127.0.0.1/").unwrap(); // to change with infra::redis

    let user_repository = Arc::new(UserRepository::new(db.clone()));
    let user_service = Arc::new(UserService::new(user_repository));
	
    let game_repository = Arc::new(GameRepository::new(db.clone()));
    let game_service = Arc::new(GameService::new(game_repository));

    let app_state = AppState {
        config,
        redis,
        user_service,
        game_service,
    };

    let app = api::build_app(app_state.clone());
    let port = app_state.config.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on {}", addr);
    println!("\nServer listening on {}\n", addr);

    axum::serve(listener, app).await.unwrap();
}
