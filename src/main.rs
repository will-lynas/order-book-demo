use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("127.0.0.1:5900").await?;
    println!("Listening on http://127.0.0.1:5900");
    
    axum::serve(listener, app).await?;
    Ok(())
}