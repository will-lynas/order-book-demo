use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5900").await?;
    println!("Listening on http://127.0.0.1:5900");
    
    axum::serve(listener, app).await?;
    Ok(())
}