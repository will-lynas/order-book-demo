#[derive(Clone, Debug)]
pub struct Entry {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Default)]
pub struct OrderBook {
    buy_entries: Vec<Entry>,
    sell_entries: Vec<Entry>,
}

impl OrderBook {
    pub fn get_first_n_entries(&self, n: usize) -> (Vec<Entry>, Vec<Entry>) {
        (
            self.buy_entries.iter().take(n).cloned().collect(),
            self.sell_entries.iter().take(n).cloned().collect(),
        )
    }

    pub fn add_buy_entry(&mut self, entry: Entry) {
        self.buy_entries.push(entry);
    }

    pub fn add_sell_entry(&mut self, entry: Entry) {
        self.sell_entries.push(entry);
    }
}
