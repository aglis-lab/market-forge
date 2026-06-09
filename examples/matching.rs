use market_forge::{core::order, matching_pool};

// Init struct
#[tokio::main]
async fn main() {
    env_logger::init();

    log::debug!("Starting matching pool example");

    let config = matching_pool::MatchingPoolConfig {
        symbols: vec![
            matching_pool::Symbol {
                symbol: "AAPL".to_string(),
                slot_idx: 0,
            },
            matching_pool::Symbol {
                symbol: "GOOG".to_string(),
                slot_idx: 1,
            },
        ],
    };
    let mut pool = matching_pool::MatchingPool::<order::OrderSpec>::new();

    pool.init(config);

    pool.push(
        0,
        order::OrderSpec::limit_price(
            0, // symbol_id for AAPL
            1001,
            order::OrderSide::Buy,
            150,
            10,
        ),
    )
    .await
    .unwrap();

    for _ in 0..100 {
        pool.push(
            1,
            order::OrderSpec::limit_price(
                1, // symbol_id for GOOG
                1002,
                order::OrderSide::Buy,
                155,
                10,
            ),
        )
        .await
        .unwrap();
    }

    pool.cancel(0);
    pool.cancel(1);

    // Wait for the cancellation task to complete before exiting
    pool.wait_all().await;
}
