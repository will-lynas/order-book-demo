use askama_axum::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::{AppState, Entry};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Buy,
    Sell,
}

#[derive(Deserialize)]
pub struct FormData {
    pub entry_type: EntryType,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Template)]
#[template(path = "table.html")]
pub struct TableTemplate {
    pub buy_entries: Vec<Entry>,
    pub sell_entries: Vec<Entry>,
}

pub async fn handler(State(state): State<AppState>, Form(form): Form<FormData>) -> TableTemplate {
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
