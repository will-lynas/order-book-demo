use askama_axum::Template;
use axum::extract::State;

use crate::{order_book::Entry, AppState};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub buy_entries: Vec<Entry>,
    pub sell_entries: Vec<Entry>,
}

pub async fn handler(State(state): State<AppState>) -> IndexTemplate {
    let order_book = state.order_book.lock().unwrap();
    IndexTemplate {
        buy_entries: order_book.buy_entries.iter().take(6).cloned().collect(),
        sell_entries: order_book.sell_entries.iter().take(6).cloned().collect(),
    }
}
