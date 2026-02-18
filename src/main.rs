use backend_axum::api;
use backend_axum::app_state::AppState;
use backend_axum::config::Config;
use backend_axum::db;
// use backend_axum::utils;

use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::parse();
    let db = db::db_init(config.database_url.clone())
        .await
        .expect("DB connection failed");
    let redis = redis::Client::open("redis://127.0.0.1/").unwrap();
    let app_state = Arc::new(AppState::new(db, config, redis));

    let app = api::build_app(app_state.clone());
    let port = app_state.get_port();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on {}", addr);
    println!("\nServer listening on {}\n", addr);

    axum::serve(listener, app).await.unwrap();
}
