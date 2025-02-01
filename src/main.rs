use anyhow::Result;
use askama_axum::Template;
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use std::{net::SocketAddr, sync::Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod handlers;

#[derive(Clone, Debug, Deserialize)]
pub struct Entry {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Clone)]
pub struct AppState {
    pub buy_entries: Arc<Mutex<Vec<Entry>>>,
    pub sell_entries: Arc<Mutex<Vec<Entry>>>,
}

fn create_router() -> Router {
    let state = AppState {
        buy_entries: Arc::new(Mutex::new(Vec::new())),
        sell_entries: Arc::new(Mutex::new(Vec::new())),
    };

    Router::new()
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
