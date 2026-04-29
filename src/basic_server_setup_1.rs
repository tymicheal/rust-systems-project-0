use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
