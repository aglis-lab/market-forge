use market_forge::core::{order, order_spec};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

/// Simulates realistic equity order flow:
///   - Mid price follows a random walk (Brownian motion)
///   - Orders cluster within 5 ticks of best bid/ask (~70% of flow)
///   - Order sizes follow power-law (many small, few large)
///   - Realistic TIF distribution (GTC dominant)
///   - Proper bid/ask spread (1–3 ticks for liquid equities)
pub fn make_realistic_orders(num_to_try: usize, seed: u64) -> Vec<order_spec::OrderSpec> {
    let mut orders = Vec::with_capacity(num_to_try);
    let mut rng = StdRng::seed_from_u64(seed);

    // --- Market state ---
    let mut mid_price: f64 = 1882.0; // starting mid between bid/ask
    let spread_ticks: i64 = 2; // typical liquid equity: 1-3 ticks
    let volatility: f64 = 0.015; // tick std-dev per order (random walk step)

    for i in 0..num_to_try {
        // 1. Evolve mid price: Gaussian random walk
        //    Each order arrival slightly moves the market
        let drift: f64 = sample_gaussian(&mut rng, 0.0, volatility);
        mid_price = (mid_price + drift).max(1.0); // floor at 1 tick

        let best_bid = (mid_price - spread_ticks as f64 / 2.0).round() as i64;
        let best_ask = best_bid + spread_ticks;

        // 2. Side distribution: slightly more buys in uptrend (51/49)
        let is_buy = rng.random_range(0u32..100) < 51;

        // 3. Price level selection — realistic clustering
        //    70% within 5 ticks, 20% within 20 ticks, 10% deeper
        let price: i64 = if is_buy {
            let offset = sample_price_offset(&mut rng); // negative = deeper in book
            (best_bid - offset).max(1)
        } else {
            let offset = sample_price_offset(&mut rng);
            best_ask + offset
        };

        // 4. Quantity: power-law distribution
        //    Most orders are small (100–300 shares), rare large blocks
        let qty = sample_quantity(&mut rng);

        let side = if is_buy {
            order::OrderSide::Buy
        } else {
            order::OrderSide::Sell
        };

        // 5. Realistic TIF distribution for equities:
        //    ~75% GTC (resting limit orders)
        //    ~15% IOC (aggressive, immediate-or-cancel)
        //    ~10% FOK (block trades, all-or-nothing)
        let time_in_force = match rng.random_range(0u32..100) {
            0..=74 => order::TimeInForce::GTC,
            75..=89 => order::TimeInForce::IOC,
            _ => order::TimeInForce::FOK,
        };

        let order = order_spec::OrderSpec::limit_price(
            i as order::OrderId,
            side,
            price as order::Price,
            qty,
        )
        .with_time_in_force(time_in_force)
        .clone();

        orders.push(order);
    }

    orders
}

/// Create `num_to_try` deterministic orders for benchmarking.
pub fn make_random_orders(num_to_try: usize, seed: u64) -> Vec<order_spec::OrderSpec> {
    let mut orders = Vec::with_capacity(num_to_try);
    let mut rng = StdRng::seed_from_u64(seed);

    for i in 0..num_to_try {
        let is_buy = i % 2 == 0;
        let delta = if is_buy { 1880 } else { 1884 };
        let multiply = if is_buy { 0 } else { 1 };
        let price = (delta + (rng.random_range(0..1000) * multiply)) as order::Price;
        let qty = ((rng.random_range(0..1000) + 1) * 100) as order::Quantity;
        let side = if is_buy {
            order::OrderSide::Buy
        } else {
            order::OrderSide::Sell
        };

        let time_in_force = match rng.random_range(0..3) {
            0 => order::TimeInForce::IOC,
            1 => order::TimeInForce::FOK,
            _ => order::TimeInForce::GTC,
        };

        let order = order_spec::OrderSpec::limit_price(i as order::OrderId, side, price, qty)
            .with_time_in_force(time_in_force)
            .clone();

        orders.push(order);
    }

    orders
}

/// Price offset from best bid/ask — heavy clustering near top of book.
///
/// Distribution (ticks away from best):
///   0–4   ticks: ~70% of orders  (active liquidity)
///   5–19  ticks: ~20% of orders  (passive liquidity)
///   20–99 ticks: ~10% of orders  (deep book / iceberg)
fn sample_price_offset(rng: &mut StdRng) -> i64 {
    match rng.random_range(0u32..100) {
        0..=69 => rng.random_range(0i64..5),   // 0-4 ticks out
        70..=89 => rng.random_range(5i64..20), // 5-19 ticks out
        _ => rng.random_range(20i64..100),     // 20-99 ticks out
    }
}

/// Order size distribution approximating equity market microstructure.
///
/// Real equity sizes roughly follow a power law:
///   ~50% orders:  100–300 shares  (retail / algo small)
///   ~30% orders:  300–1000 shares (mid-size)
///   ~15% orders:  1000–5000 shares (institutional)
///   ~5%  orders:  5000–50000 shares (block)
fn sample_quantity(rng: &mut StdRng) -> order::Quantity {
    let raw = match rng.random_range(0u32..100) {
        0..=49 => rng.random_range(1u64..4) * 100,    // 100–300
        50..=79 => rng.random_range(3u64..11) * 100,  // 300–1000
        80..=94 => rng.random_range(10u64..51) * 100, // 1000–5000
        _ => rng.random_range(50u64..501) * 100,      // 5000–50000
    };
    raw as order::Quantity
}

/// Box-Muller transform — Gaussian sample without pulling in rand_distr.
fn sample_gaussian(rng: &mut StdRng, mean: f64, std_dev: f64) -> f64 {
    let u1: f64 = rng.random_range(1e-10f64..1.0); // avoid log(0)
    let u2: f64 = rng.random_range(0.0f64..1.0);
    let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    mean + std_dev * z
}
