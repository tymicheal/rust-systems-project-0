use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tokio;
use tower::ServiceExt;

async fn app() -> Router {
    Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }))
}

#[tokio::test]
async fn test_root_route() {
    let app = app().await;
    let response = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, World!");
}

#[tokio::test]
async fn test_not_found_route() {
    let app = app().await;
    let response = app
        .oneshot(Request::get("/nonexistent").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
