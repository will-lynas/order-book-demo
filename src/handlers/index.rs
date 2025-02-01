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
    let (buy_entries, sell_entries) = order_book.get_first_n_entries(6);
    IndexTemplate {
        buy_entries,
        sell_entries,
    }
}
