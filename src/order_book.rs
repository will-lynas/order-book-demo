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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_entries_maintain_max_heap_order() {
        let mut order_book = OrderBook::default();

        // Add entries in arbitrary order
        order_book.add_buy_entry(Entry {
            price: 100.0,
            quantity: 1.0,
        });
        order_book.add_buy_entry(Entry {
            price: 150.0,
            quantity: 2.0,
        });
        order_book.add_buy_entry(Entry {
            price: 50.0,
            quantity: 3.0,
        });
        order_book.add_buy_entry(Entry {
            price: 200.0,
            quantity: 4.0,
        });

        let (buy_entries, _) = order_book.get_first_n_entries(5);

        let expected_prices = vec![200.0, 150.0, 100.0, 50.0];
        assert_eq!(buy_entries.len(), expected_prices.len());

        for (entry, &expected_price) in buy_entries.iter().zip(expected_prices.iter()) {
            assert_eq!(
                entry.price, expected_price,
                "Expected price {} but got {}",
                expected_price, entry.price
            );
        }
    }
}
