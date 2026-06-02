use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use fake::{
    Rng,
    rand::{SeedableRng, rngs::StdRng},
};
use market_forge::core::{
    order::{Order, OrderId, OrderSide, Price, Quantity, TimeInForce},
    order_book::OrderBook,
    order_spec::OrderSpec,
};
use std::mem::size_of;
use std::time::{Duration, Instant};

const SIZES: [usize; 10] = [
    500_000usize,
    1_000_000usize,
    3_000_000usize,
    5_000_000usize,
    6_000_000usize,
    7_000_000usize,
    8_000_000usize,
    9_000_000usize,
    10_000_000usize,
    11_000_000usize,
];

/// Create `num_to_try` deterministic orders for benchmarking.
fn make_orders(num_to_try: usize, seed: u64) -> Vec<OrderSpec> {
    let mut orders = Vec::with_capacity(num_to_try);
    let mut rng = StdRng::seed_from_u64(seed);

    for i in 0..num_to_try {
        let is_buy = i % 2 == 0;
        let delta = if is_buy { 1880 } else { 1884 };
        let multiply = if is_buy { 0 } else { 1 };
        let price = (delta + (rng.random_range(0..1000) * multiply)) as Price;
        let qty = ((rng.random_range(0..1000) + 1) * 100) as Quantity;
        let side = if is_buy {
            OrderSide::Buy
        } else {
            OrderSide::Sell
        };

        let time_in_force = match rng.random_range(0..3) {
            0 => TimeInForce::IOC,
            1 => TimeInForce::FOK,
            _ => TimeInForce::GTC,
        };

        let order = OrderSpec::limit_price(i as OrderId, side, price, qty)
            .with_time_in_force(time_in_force)
            .clone();

        orders.push(order);
    }

    orders
}

fn insert_orders_once(orders: &[OrderSpec]) {
    let mut book = OrderBook::<OrderSpec>::new(orders.len());
    for order in orders {
        let _ = book.insert_order(order);
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn insert_then_cancel_once(orders: &[OrderSpec]) {
    let mut book = OrderBook::<OrderSpec>::new(orders.len());
    for order in orders {
        let _ = book.insert_order(order);
    }
    for order in orders {
        let _ = book.cancel_order(&OrderSpec::cancel(order.id, order.order_side, order.price));
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn bench_perf_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf_order_insert");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(20);

    for &num in &SIZES {
        let orders = make_orders(num, num as u64);
        let order_bytes = (size_of::<OrderSpec>() as u64) * (num as u64);
        group.throughput(Throughput::Bytes(order_bytes));

        group.bench_with_input(BenchmarkId::from_parameter(num), &num, |b, &_num| {
            b.iter_custom(|iters| {
                let start = Instant::now();
                for _ in 0..iters {
                    insert_orders_once(&orders);
                }
                start.elapsed()
            })
        });
    }

    group.finish();
}

fn bench_perf_cancel(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf_order_cancel");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(20);

    for &num in &SIZES {
        let num = num / 2; // We do an insert + cancel for each order; cut num in half to keep runtime reasonable
        let orders = make_orders(num, num as u64 + 1);
        // We do an insert + cancel for each order; count bytes twice per order
        let order_bytes = (size_of::<OrderSpec>() as u64) * (num as u64) * 2;
        group.throughput(Throughput::Bytes(order_bytes));

        group.bench_with_input(BenchmarkId::from_parameter(num), &num, |b, &_num| {
            b.iter_custom(|iters| {
                let start = Instant::now();
                for _ in 0..iters {
                    insert_then_cancel_once(&orders);
                }
                start.elapsed()
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_perf_insert, bench_perf_cancel);

criterion_main!(benches);

#[test]
fn verify_bench_file_runs() {
    // This test verifies the bench file is discovered by the test harness.
    assert!(true);
}
