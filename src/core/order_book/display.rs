use crate::core::{order, order_book::OrderBook};
use tabled::{builder::Builder, settings::Style};

impl<T: order::Order> std::fmt::Display for OrderBook<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = writeln!(
            f,
            "Bids: {}, Bids Qty: {}, Asks: {}, Asks: Qty: {}, Alloc: {}",
            self.bids.len(),
            self.bids.total_quantity(),
            self.asks.len(),
            self.asks.total_quantity(),
            self.order_allocator.len()
        );

        let mut builder = Builder::new();
        builder.push_record(["Bids", "Total", "Asks", "Total"]);

        // Reverse bids for descending order (as bid books are usually displayed)
        let bids: Vec<_> = self.bids.orders().iter().collect();
        let asks: Vec<_> = self.asks.orders().iter().collect();
        let max_len = bids.len().max(asks.len());

        for i in 0..max_len {
            let (bid_price, bid_qty) = bids
                .get(i)
                .map(|(p, o)| {
                    (
                        p.0.to_string(),
                        format!("{}({})", o.orders_quantity().to_string(), o.len()),
                    )
                })
                .unwrap_or(("".to_string(), "".to_string()));

            let (ask_price, ask_qty) = asks
                .get(i)
                .map(|(p, o)| {
                    (
                        p.to_string(),
                        format!("{}({})", o.orders_quantity().to_string(), o.len()),
                    )
                })
                .unwrap_or(("".to_string(), "".to_string()));

            builder.push_record([bid_price, bid_qty, ask_price, ask_qty]);
        }

        let mut table = builder.build();
        let temp = table.with(Style::modern_rounded());
        write!(f, "{temp}")
    }
}
