#[derive(Clone, Debug)]
pub struct Entry {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Default)]
pub struct OrderBook {
    pub buy_entries: Vec<Entry>,
    pub sell_entries: Vec<Entry>,
}
