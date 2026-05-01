use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    id: u64,
    name: String,
    email: String,
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> impl IntoResponse {
    // Basic validation
    if payload.name.is_empty() {
        return (StatusCode::BAD_REQUEST, "Name cannot be empty").into_response();
    }
    
    if !payload.email.contains('@') {
        return (StatusCode::BAD_REQUEST, "Invalid email format").into_response();
    }
    
    // Simulate creating a user
    let user = CreateUserResponse {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    
    (StatusCode::CREATED, Json(user)).into_response()
}

async fn get_users() -> impl IntoResponse {
    Json(vec!["user1", "user2"])
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(create_user).get(get_users));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
