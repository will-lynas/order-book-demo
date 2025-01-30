use anyhow::Result;
use askama_axum::Template;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "click_response.html")]
struct ClickResponseTemplate {}

async fn index_handler() -> IndexTemplate {
    IndexTemplate {}
}

async fn click_handler() -> ClickResponseTemplate {
    ClickResponseTemplate {}
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/clicked", post(click_handler))
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
