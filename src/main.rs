use anyhow::Result;
use askama_axum::Template;
use axum::{
    extract::State,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use std::{net::SocketAddr, sync::Mutex};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone, Debug, Deserialize)]
struct Entry {
    price: f64,
    quantity: f64,
    entry_type: EntryType,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum EntryType {
    Buy,
    Sell,
}

#[derive(Clone)]
struct AppState {
    entries: Arc<Mutex<Vec<Entry>>>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<Entry>,
}

async fn index_handler(State(state): State<AppState>) -> IndexTemplate {
    let entries = state.entries.lock().unwrap().clone();
    IndexTemplate { entries }
}

async fn submit_entry_handler(
    State(state): State<AppState>,
    Form(form): Form<Entry>,
) -> TableTemplate {
    let mut entries = state.entries.lock().unwrap();
    entries.push(form);

    TableTemplate {
        entries: entries.clone(),
    }
}

#[derive(Template)]
#[template(path = "table.html")]
struct TableTemplate {
    entries: Vec<Entry>,
}

fn create_router() -> Router {
    let state = AppState {
        entries: Arc::new(Mutex::new(Vec::new())),
    };

    Router::new()
        .route("/", get(index_handler))
        .route("/submit-entry", post(submit_entry_handler))
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
