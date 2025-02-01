use anyhow::Result;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod handlers;
mod order_book;

use order_book::OrderBook;

#[derive(Clone, Default)]
pub struct AppState {
    pub order_book: Arc<Mutex<OrderBook>>,
}

fn create_router() -> Router {
    let state = AppState::default();

    Router::new()
        .route("/favicon.ico", get(|| async { StatusCode::NO_CONTENT }))
        .route("/", get(handlers::index::handler))
        .route("/submit-entry", post(handlers::submit::handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
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
