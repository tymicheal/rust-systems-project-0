use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
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
    id: u32,
    name: String,
    email: String,
}

async fn create_user(
    State(users): State<Vec<User>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Name cannot be empty"})),
        ));
    }

    if !payload.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid email format"})),
        ));
    }

    let new_user = User {
        id: users.len() as u32 + 1,
        name: payload.name,
        email: payload.email,
    };

    Ok((StatusCode::CREATED, Json(new_user)))
}

async fn get_users() -> impl IntoResponse {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    Json(users)
}

#[tokio::main]
async fn main() {
    let users = vec![];
    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .with_state(users);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
