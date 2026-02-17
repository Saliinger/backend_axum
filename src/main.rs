use backend_axum::api;
use backend_axum::app_state;
use backend_axum::app_state::AppState;
use backend_axum::config::Config;
use backend_axum::utils;
use clap::Parser;

use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::parse();
    let app_state = AppState::new(config).await?;

    let app = api::build_app(app_state);
    let port = app_state.get_port();

    let addr = SocketAddr::from(([0.0.0.0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on {}", addr);
    println!("\nServer listening on {}\n", addr);

    axum::serve(listener, app).await.unwrap();
}
