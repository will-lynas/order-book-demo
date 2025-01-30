use anyhow::Result;
use askama_axum::Template;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index_handler() -> IndexTemplate {
    IndexTemplate {}
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(index_handler));

    let listener = TcpListener::bind("127.0.0.1:5900").await?;
    println!("Listening on http://127.0.0.1:5900");

    axum::serve(listener, app).await?;
    Ok(())
}
