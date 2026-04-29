use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({"message": "Hello, World!"}))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({"status": "healthy"}))
}
