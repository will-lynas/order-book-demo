use anyhow::Result;
use askama_axum::Template;
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicU64, Ordering},
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    counter: Arc<AtomicU64>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    count: u64,
}

#[derive(Template)]
#[template(path = "click_response.html")]
struct ClickResponseTemplate {
    count: u64,
}

async fn index_handler(State(state): State<AppState>) -> IndexTemplate {
    let count = state.counter.load(Ordering::Relaxed);
    IndexTemplate { count }
}

async fn click_handler(State(state): State<AppState>) -> ClickResponseTemplate {
    let count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;
    ClickResponseTemplate { count }
}

fn create_router() -> Router {
    // Create shared state
    let state = AppState {
        counter: Arc::new(AtomicU64::new(0)),
    };

    Router::new()
        .route("/", get(index_handler))
        .route("/clicked", post(click_handler))
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
