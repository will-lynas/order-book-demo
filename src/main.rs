use anyhow::Result;
use askama_axum::Template;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index_handler() -> IndexTemplate {
    IndexTemplate {}
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .nest_service("/static", ServeDir::new("static"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = create_router();

    let addr: SocketAddr = "127.0.0.1:5900".parse()?;
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
