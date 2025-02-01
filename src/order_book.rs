use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub price: f64,
    pub quantity: f64,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.total_cmp(&other.price)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct OrderBook {
    buy_entries: BinaryHeap<Entry>,
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
