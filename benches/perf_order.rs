use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use fake::{
    Rng,
    rand::{SeedableRng, rngs::StdRng},
};
use market_forge::core::{
    order::Order,
    order_book::{self, OrderBook},
    order_spec::{self, OrderSpec},
};
use std::time::{Duration, Instant};

use stats_alloc::{INSTRUMENTED_SYSTEM, StatsAlloc};
use std::alloc::System;

mod simulate_order;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

const SAMPLE_SIZE: usize = 10;

const SIZES_PERF_CANCEL: [usize; 12] = [
    100_000usize,
    200_000usize,
    300_000usize,
    400_000usize,
    600_000usize,
    800_000usize,
    1_000_000usize,
    2_000_000usize,
    5_000_000usize,
    10_000_000usize,
    20_000_000usize,
    30_000_000usize,
];

const SIZES_PERF_REPLACE: [usize; 12] = [
    100_000usize,
    200_000usize,
    300_000usize,
    400_000usize,
    600_000usize,
    800_000usize,
    1_000_000usize,
    2_000_000usize,
    5_000_000usize,
    10_000_000usize,
    20_000_000usize,
    30_000_000usize,
];

const SIZES_PERF_MATCHING: [usize; 12] = [
    100_000usize,
    200_000usize,
    300_000usize,
    400_000usize,
    600_000usize,
    800_000usize,
    1_000_000usize,
    2_000_000usize,
    5_000_000usize,
    10_000_000usize,
    20_000_000usize,
    30_000_000usize,
];

const SIZES_PERF_COMBINED: [usize; 12] = [
    100_000usize,
    200_000usize,
    300_000usize,
    400_000usize,
    600_000usize,
    800_000usize,
    1_000_000usize,
    2_000_000usize,
    5_000_000usize,
    10_000_000usize,
    20_000_000usize,
    30_000_000usize,
];

fn bench_perf_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf_order_matching");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(SAMPLE_SIZE);

    for &num in &SIZES_PERF_MATCHING {
        let orders = simulate_order::make_realistic_orders(num, num as u64);
        group.throughput(Throughput::Elements(num as u64));

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
    group.sample_size(SAMPLE_SIZE);

    for &num in &SIZES_PERF_CANCEL {
        let mut book = OrderBook::<order_spec::OrderSpec>::default();
        let orders = simulate_order::make_realistic_orders(num, num as u64 + 1);

        // Count both insert and cancel operations
        group.throughput(Throughput::Elements(num as u64));

        group.bench_with_input(BenchmarkId::from_parameter(num), &num, |b, &_num| {
            b.iter_custom(|iters| {
                insert_only(&mut book, &orders);
                let start = Instant::now();
                cancel_only(&mut book, &orders);
                start.elapsed()
            })
        });
    }

    group.finish();
}

fn bench_perf_replace(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf_order_replace");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(SAMPLE_SIZE);

    for &num in &SIZES_PERF_REPLACE {
        let mut book = OrderBook::<order_spec::OrderSpec>::default();
        let orders = simulate_order::make_realistic_orders(num, num as u64 + 2);
        // Count both insert and replace operations
        group.throughput(Throughput::Elements((num as u64) * 2));

        group.bench_with_input(BenchmarkId::from_parameter(num), &num, |b, &_num| {
            b.iter_custom(|iters| {
                let mut total_elapsed = Duration::new(0, 0);
                for _ in 0..iters {
                    insert_only(&mut book, &orders);
                    let start: Instant = Instant::now();
                    replace_only(&mut book, &orders);
                    total_elapsed += start.elapsed();
                }
                total_elapsed
            })
        });
    }

    group.finish();
}

fn bench_perf_combine(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf_order_combined");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(SAMPLE_SIZE);

    // Chosen ratio: inserts always, replacements ~20%, cancels ~10%.
    // This models a heavy-insert workload with occasional modifications/cancels.
    let replace_pct = 20u32;
    let cancel_pct = 10u32;
    let rep_f = (replace_pct as f64) / 100.0;
    let can_f = (cancel_pct as f64) / 100.0;

    for &num in &SIZES_PERF_COMBINED {
        let mut book = OrderBook::<OrderSpec>::with_capacity(num);
        let orders = simulate_order::make_realistic_orders(num, num as u64 + 3);
        let expected_ops = ((num as f64) * (1.0 + rep_f + can_f)).round() as u64;
        group.throughput(Throughput::Elements(expected_ops));

        group.bench_with_input(BenchmarkId::from_parameter(num), &num, |b, &_num| {
            b.iter_custom(|iters| {
                let start = Instant::now();
                for _ in 0..iters {
                    insert_replace_cancel_once(
                        &mut book,
                        &orders,
                        replace_pct,
                        cancel_pct,
                        num as u64 + 4,
                    );
                }
                start.elapsed()
            })
        });
    }

    group.finish();
}

fn insert_orders_once(orders: &[OrderSpec]) {
    let mut book = OrderBook::<OrderSpec>::default();
    for order in orders {
        let _ = book.insert_order(order);
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn insert_only(book: &mut OrderBook<OrderSpec>, orders: &[OrderSpec]) {
    for order in orders {
        let _ = book.insert_order(order);
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn cancel_only(book: &mut OrderBook<OrderSpec>, orders: &[OrderSpec]) {
    for order in orders {
        let _ = book.cancel_order(order.id());
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn insert_then_cancel(book: &mut OrderBook<OrderSpec>, orders: &[OrderSpec]) {
    for order in orders {
        let _ = book.insert_order(order);
    }

    for order in orders {
        let _ = book.cancel_order(order.id());
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn replace_only(book: &mut OrderBook<OrderSpec>, orders: &[OrderSpec]) {
    for order in orders {
        let new_price = order.price + 10;
        let quantity_delta = -50;
        let _ = book.replace_order(order_book::ReplaceOrder {
            order_id: order.id(),
            new_price,
            quantity_delta,
        });
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn insert_then_replace_once(orders: &[OrderSpec]) {
    let mut book = OrderBook::<OrderSpec>::default();
    for order in orders {
        let _ = book.insert_order(order);
    }
    for order in orders {
        let new_price = order.price + 10;
        let quantity_delta = -50;
        let _ = book.replace_order(order_book::ReplaceOrder {
            order_id: order.id(),
            new_price,
            quantity_delta,
        });
    }
    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

fn insert_replace_cancel_once(
    book: &mut OrderBook<OrderSpec>,
    orders: &[OrderSpec],
    replace_pct: u32,
    cancel_pct: u32,
    rng_seed: u64,
) {
    let mut rng = StdRng::seed_from_u64(rng_seed);

    for order in orders {
        let _ = book.insert_order(order);

        let r = rng.random_range(0..100u32);
        if r < replace_pct {
            let new_price = order.price + 10;
            let quantity_delta = -50;

            let _ = book.replace_order(order_book::ReplaceOrder {
                order_id: order.id(),
                new_price,
                quantity_delta,
            });
        } else if r < replace_pct + cancel_pct {
            let _ = book.cancel_order(order.id());
        }
    }

    if let Some(err) = book.validate_cache().err() {
        panic!("{:?}", err);
    }
}

criterion_group!(
    benches,
    bench_perf_matching,
    bench_perf_cancel,
    bench_perf_replace,
    bench_perf_combine
);

criterion_main!(benches);

#[test]
fn verify_bench_file_runs() {
    // This test verifies the bench file is discovered by the test harness.
    assert!(true);
}
