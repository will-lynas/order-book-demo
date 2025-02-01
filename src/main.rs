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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    buy_entries: Vec<Entry>,
    sell_entries: Vec<Entry>,
}

async fn index_handler(State(state): State<AppState>) -> IndexTemplate {
    let buy_entries = state.buy_entries.lock().unwrap().clone();
    let sell_entries = state.sell_entries.lock().unwrap().clone();
    IndexTemplate {
        buy_entries: buy_entries.into_iter().take(6).collect(),
        sell_entries: sell_entries.into_iter().take(6).collect(),
    }
}

fn create_router() -> Router {
    let state = AppState {
        buy_entries: Arc::new(Mutex::new(Vec::new())),
        sell_entries: Arc::new(Mutex::new(Vec::new())),
    };

    Router::new()
        .route("/", get(index_handler))
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
