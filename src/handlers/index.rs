use askama_axum::Template;
use axum::extract::State;

use crate::{AppState, Entry};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub buy_entries: Vec<Entry>,
    pub sell_entries: Vec<Entry>,
}

pub async fn handler(State(state): State<AppState>) -> IndexTemplate {
    let buy_entries = state.buy_entries.lock().unwrap().clone();
    let sell_entries = state.sell_entries.lock().unwrap().clone();
    IndexTemplate {
        buy_entries: buy_entries.into_iter().take(6).collect(),
        sell_entries: sell_entries.into_iter().take(6).collect(),
    }
}
