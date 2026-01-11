mod config;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::Config;

#[tokio::main]
async fn main() {
    // 1. Initialize Logging/Tracing
    // This will print logs to the console based on RUST_LOG env var
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "saweria_webhook=info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Load Configuration
    let config = Config::from_env();

    // 3. Build Application Router
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .route(&config.webhook_path, post(handlers::handle_webhook));

    // 4. Start the Server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Server started on http://{}", addr);
    tracing::info!("listening for webhooks at: {}", config.webhook_path);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
