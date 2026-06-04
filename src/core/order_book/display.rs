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
        builder.push_record([
            "Bid Price",
            "Bid Qty",
            "Bid Head",
            "Bid Tail",
            "Bid Level Len",
            "Ask Price",
            "Ask Qty",
            "Ask Head",
            "Ask Tail",
            "Ask Level Len",
        ]);

        // Reverse bids for descending order (as bid books are usually displayed)
        let bids: Vec<_> = self.bids.iter().collect();
        let asks: Vec<_> = self.asks.iter().collect();
        let max_len = bids.len().max(asks.len());

        for i in 0..max_len {
            let (bid_price, bid_qty, bid_head, bid_tail, bid_level_length) = bids
                .get(i)
                .map(|(p, level)| {
                    (
                        p.0.to_string(),
                        format!("{}({})", level.quantity().to_string(), level.len()),
                        level.head().to_string(),
                        level.tail().to_string(),
                        level.len().to_string(),
                    )
                })
                .unwrap_or((
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ));

            let (ask_price, ask_qty, ask_head, ask_tail, ask_level_length) = asks
                .get(i)
                .map(|(p, level)| {
                    (
                        p.to_string(),
                        format!("{}({})", level.quantity().to_string(), level.len()),
                        level.head().to_string(),
                        level.tail().to_string(),
                        level.len().to_string(),
                    )
                })
                .unwrap_or((
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ));

            builder.push_record([
                bid_price,
                bid_qty,
                bid_head,
                bid_tail,
                bid_level_length,
                ask_price,
                ask_qty,
                ask_head,
                ask_tail,
                ask_level_length,
            ]);
        }

        let mut table = builder.build();
        let temp = table.with(Style::modern_rounded());
        write!(f, "{temp}")?;

        // Display Order Allocator
        write!(f, "{}", self.order_allocator)?;

        Ok(())
    }
}
