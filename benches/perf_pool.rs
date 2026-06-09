use async_ringbuf::traits::{AsyncProducer, Producer};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use market_forge::{core::order::OrderSpec, matching_pool};
use std::time::Duration;

mod simulate_order;

// 1000 packets at once is best for high throughput
const SIZES_THROUGHPUT: [usize; 3] = [10_000_000, 15_000_000, 30_000_000];
const SIZES_SYMBOLS: [usize; 5] = [1, 2, 5, 8, 10];

fn bench_perf_pool(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("perf_pool_push");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(10);

    for &num_symbols in &SIZES_SYMBOLS {
        for &num in &SIZES_THROUGHPUT {
            let orders = std::sync::Arc::new(simulate_order::make_realistic_orders(
                num,
                num_symbols,
                num as u64,
            ));

            group.throughput(Throughput::Elements(num as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}-{}", num_symbols, num)),
                &(num_symbols, num),
                |b, _| {
                    b.to_async(&rt).iter(|| {
                        let orders = orders.clone();

                        async move {
                            insert_orders(orders.as_slice(), num_symbols).await;
                        }
                    });
                },
            );
        }
    }

    group.finish();
}

async fn insert_orders(orders: &[OrderSpec], num_symbols: usize) {
    let mut pool = matching_pool::MatchingPool::<OrderSpec>::new();
    let mut symbols = Vec::with_capacity(num_symbols);
    for index in 0..num_symbols {
        symbols.push(matching_pool::Symbol {
            symbol: format!("SYM{}", index),
            slot_idx: index,
        });
    }

    pool.init(matching_pool::MatchingPoolConfig { symbols: symbols });
    let mut err_count = 0;
    for order in orders.iter() {
        if let Err(_) = pool
            .get_producer(order.symbol_id as usize)
            .expect("Producer not found for symbol_id")
            .push(order.clone())
        {
            err_count += 1;
        }
    }

    for index in 0..num_symbols {
        pool.cancel(index);
    }

    pool.wait_all().await;

    if err_count > 0 {
        println!(
            "Inserted {} orders at num_symbols {} with {} errors",
            orders.len(),
            num_symbols,
            err_count
        );
    }
}

criterion_group!(benches, bench_perf_pool);
criterion_main!(benches);
