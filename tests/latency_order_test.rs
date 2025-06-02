#[cfg(test)]
mod tests {
    use std::time::Instant;

    use fake::{
        Rng,
        rand::{SeedableRng, rngs::StdRng},
    };
    use market_forge::core::{
        order::{OrderId, OrderSide, Price, Quantity},
        order_book::OrderBook,
        order_spec::OrderSpec,
    };

    #[test]
    fn latency_order_book_test() {
        let num_to_try = 10_000_000;
        let mut book = OrderBook::<OrderSpec>::new(1_500_000);
        let mut timestamps = Vec::with_capacity(num_to_try + 1);

        // Use a fixed seed for reproducibility
        let mut rng = StdRng::seed_from_u64(num_to_try as u64);

        // Generate and insert orders, recording timestamps
        for i in 0..num_to_try {
            let is_buy = i % 2 == 0;
            let delta = if is_buy { 1880 } else { 1884 };
            let price = (rng.random_range(0..10) + delta) as Price;
            let qty = ((rng.random_range(0..10) + 1) * 100) as Quantity;
            let side = if is_buy {
                OrderSide::Buy
            } else {
                OrderSide::Sell
            };
            let order = OrderSpec::limit_price(i as OrderId, side, price, qty);

            let now = Instant::now();
            book.insert_order(&order);
            timestamps.push(now);
        }
        // Final timestamp
        let end = Instant::now();
        timestamps.push(end);

        // Print latency histogram
        println!("Latency (ns):");
        for w in timestamps.windows(2) {
            let elapsed = w[1].duration_since(w[0]).as_nanos();
            println!("{}ns", elapsed);
        }

        // Calculate average latency
        let durations: Vec<_> = timestamps
            .windows(2)
            .map(|w| w[1].duration_since(w[0]))
            .collect();
        let total: std::time::Duration = durations.iter().sum();
        let avg = total / durations.len() as u32;
        println!(
            "Total: {}ms\tAverage: {}ns",
            total.as_millis(),
            avg.as_nanos()
        );
    }
}
