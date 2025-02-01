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

impl OrderBook {
    pub fn get_first_n_entries(&self, n: usize) -> (Vec<Entry>, Vec<Entry>) {
        (
            self.buy_entries.iter().take(n).cloned().collect(),
            self.sell_entries.iter().take(n).cloned().collect(),
        )
    }
}
