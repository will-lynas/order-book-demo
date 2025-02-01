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
        self.buy_entries.sort_by(|a, b| b.price.total_cmp(&a.price));
    }

    pub fn add_sell_entry(&mut self, entry: Entry) {
        self.sell_entries.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_entries_maintain_sorted_order() {
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
