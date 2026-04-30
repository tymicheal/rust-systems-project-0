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

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_not_found_route() {
    let app = Router::new().route("/", axum::routing::get(|| async { "Hello World" }));

    let response = app
        .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_hello_route() {
    let app = Router::new().route(
        "/hello",
        axum::routing::get(|| async { "Hello, World!" }),
    );

    let response = app
        .oneshot(Request::builder().uri("/hello").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(body.to_vec(), b"Hello, World!");
}
