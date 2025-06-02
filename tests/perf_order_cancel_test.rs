use fake::{
    Rng,
    rand::{SeedableRng, rngs::StdRng},
};
use market_forge::core::{
    order::{Order, OrderId, OrderSide, Price, Quantity, TimeInForce},
    order_book::OrderBook,
    order_spec::OrderSpec,
};
use std::time::{Duration, Instant};

fn run_perf_test(duration_secs: u64, num_to_try: usize) -> bool {
    println!("Trying run of {num_to_try} orders");
    let mut book = OrderBook::<OrderSpec>::new(45_000_000);
    let mut orders = Vec::with_capacity(num_to_try);

    // Use a fixed seed for reproducibility
    let mut rng = StdRng::seed_from_u64(duration_secs);

    for i in 0..num_to_try {
        let is_buy = i % 2 == 0;
        let delta = if is_buy { 1880 } else { 1884 };
        let price = (rng.random_range(0..200) + delta) as Price;
        let qty = ((rng.random_range(0..100) + 1) * 100) as Quantity;
        let side = if is_buy {
            OrderSide::Buy
        } else {
            OrderSide::Sell
        };

        let time_in_force = {
            let type_force = rng.random_range(0..3);

            if type_force == 0 {
                TimeInForce::IOC
            } else if type_force == 1 {
                TimeInForce::FOK
            } else {
                TimeInForce::GTC
            }
        };
        let is_limit = rng.random_bool(0.5);

        let order: OrderSpec;
        if is_limit {
            order = OrderSpec::limit_price(i as OrderId, side, price, qty)
                .with_time_in_force(time_in_force)
                .clone();
        } else {
            order = OrderSpec::market(i as OrderId, side, qty);
        }

        orders.push(order);
    }

    let start = Instant::now();
    let stop = start + Duration::from_secs(duration_secs);

    let mut is_max_order = false;
    let mut order_count = 0;
    for order in &orders {
        if Instant::now() >= stop {
            is_max_order = true;
            break;
        }

        _ = book.insert_order(order);
        order_count += 1;
    }

    for order in &orders {
        if Instant::now() >= stop {
            is_max_order = true;
            break;
        }

        _ = book.cancel_order(&OrderSpec::cancel(order.id, order.order_side, order.price));
        order_count += 1;
    }

    if is_max_order {
        println!(" - complete!");
        println!(
            "Inserted {} orders in {} seconds, or {} insertions per sec",
            order_count,
            duration_secs,
            order_count as u64 / duration_secs
        );
        let remain: usize = book.bids().len() + book.asks().len();
        println!("Run matched {} orders", order_count - remain);
    } else {
        println!(" - not enough orders");
    }

    // Check validation
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }

    return is_max_order;
}

#[test]
fn perf_order_book_test() {
    let duration_secs = 3;
    let base_try = duration_secs * 500_000;

    for i in 1..20 {
        let num_to_try = i * base_try;

        if run_perf_test(duration_secs, num_to_try as usize) {
            break;
        }
    }
}
