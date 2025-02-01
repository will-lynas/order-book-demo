use askama_axum::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::{order_book::Entry, AppState};

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

    let mut order_book = state.order_book.lock().unwrap();
    match form.entry_type {
        EntryType::Buy => order_book.buy_entries.push(entry),
        EntryType::Sell => order_book.sell_entries.push(entry),
    }

    let (buy_entries, sell_entries) = order_book.get_first_n_entries(6);
    TableTemplate {
        buy_entries,
        sell_entries,
    }
}
