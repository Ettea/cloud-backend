mod routes;
mod models;
mod db;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tracing_subscriber;
use routes::telemetry::post_telemetry;
use routes::telemetry::get_telemetry;


#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Try to connect to database
    let pool = db::get_db_pool().await.ok();
    if let Some(ref p) = pool {
        if let Err(e) = db::init_db(p).await {
            tracing::error!("Failed to run migrations: {:?}", e);
        }
    } else {
        tracing::warn!("Database not available, starting without DB connection");
    }

    let app_state = models::AppState { pool };

    // Build the application with routes
    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/telemetry", post(post_telemetry).get(get_telemetry))
        .with_state(app_state);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
