use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tokio;
use tower::ServiceExt;

#[tokio::test]
async fn test_root_route() {
    let app = Router::new().route("/", axum::routing::get(|| async { "Hello World" }));
    
    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_route() {
    let app = Router::new().route("/health", axum::routing::get(|| async { "OK" }));
    
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_not_found() {
    let app = Router::new().route("/", axum::routing::get(|| async { "Hello World" }));
    
    let request = Request::builder()
        .uri("/nonexistent")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
