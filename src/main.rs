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
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum EntryType {
    Buy,
    Sell,
}

#[derive(Deserialize)]
struct FormData {
    entry_type: EntryType,
    price: f64,
    quantity: f64,
}

#[derive(Clone)]
struct AppState {
    buy_entries: Arc<Mutex<Vec<Entry>>>,
    sell_entries: Arc<Mutex<Vec<Entry>>>,
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

async fn submit_entry_handler(
    State(state): State<AppState>,
    Form(form): Form<FormData>,
) -> TableTemplate {
    let entry = Entry {
        price: form.price,
        quantity: form.quantity,
    };

    let entries = match form.entry_type {
        EntryType::Buy => &state.buy_entries,
        EntryType::Sell => &state.sell_entries,
    };
    entries.lock().unwrap().push(entry);

    let buy_entries = state.buy_entries.lock().unwrap().clone();
    let sell_entries = state.sell_entries.lock().unwrap().clone();

    TableTemplate {
        buy_entries: buy_entries.into_iter().take(6).collect(),
        sell_entries: sell_entries.into_iter().take(6).collect(),
    }
}

#[derive(Template)]
#[template(path = "table.html")]
struct TableTemplate {
    buy_entries: Vec<Entry>,
    sell_entries: Vec<Entry>,
}

fn create_router() -> Router {
    let state = AppState {
        buy_entries: Arc::new(Mutex::new(Vec::new())),
        sell_entries: Arc::new(Mutex::new(Vec::new())),
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
