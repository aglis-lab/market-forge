use fake::{
    Rng,
    rand::{SeedableRng, rngs::StdRng},
};
use market_forge::{
    order::{OrderSide, Price, Quantity},
    order_book::OrderBook,
    order_default::OrderDefault,
};
use std::time::{Duration, Instant};

fn run_perf_test(duration_secs: u64, num_to_try: usize) {
    println!("Trying run of {num_to_try} orders");
    let mut book = OrderBook::<OrderDefault>::new();
    let mut orders = Vec::with_capacity(num_to_try);

    // Use a fixed seed for reproducibility
    let mut rng = StdRng::seed_from_u64(duration_secs);

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
        let order = OrderDefault::new(side, i as u64, price, qty);
        orders.push(order);
    }

    let start = Instant::now();
    let stop = start + Duration::from_secs(duration_secs);

    let mut count = 0;
    for order in &orders {
        if Instant::now() >= stop {
            break;
        }
        book.add(order);
        count += 1;
    }

    if count > 0 {
        println!(" - complete!");
        println!(
            "Inserted {} orders in {} seconds, or {} insertions per sec",
            count,
            duration_secs,
            count as u64 / duration_secs
        );
        let remain = book.bids().len() + book.asks().len();
        println!("Run matched {} orders", count - remain);
    } else {
        println!(" - not enough orders");
    }
}

#[test]
fn perf_order_book_test() {
    let duration_secs = 3;
    let mut num_to_try = duration_secs * 125_000;

    for _ in 0..10 {
        num_to_try *= 2;

        run_perf_test(duration_secs, num_to_try as usize);
    }
}
