# Market Forge

**Market Forge** is a high-performance, Rust-based matching engine. This project aims to provide a modern, memory-safe, and efficient core for building financial exchanges and trading platforms.

## 🚀 Why Market Forge?

- ✨ Written in **Rust** for safety, concurrency, and performance
- 🔁 Efficient **order book** matching logic
- 🧩 Modular and easy to integrate into trading systems
- ♻️ Seems clean for me, hahahh

## 📦 Features

- Price-time priority matching
- Supports limit, market, cancel orders, and replace orders
- Unit-tested and performance-optimized core

## 📚 Example

### Order Spec Implementation

You can implement your own Order trait but the default should be already sufficient enough for most usecases.

**Memory Layout & Performance:**

- `OrderSpec` is **32 bytes** in size.
- Each order transaction consumes roughly **40-56 bytes** in memory (including allocator overhead).
- Benchmark results: **10-14M transactions/sec** on combined workloads, demonstrating highly efficient memory utilization and cache locality.

```rust
use market_forge::{order::OrderSide, order_book::OrderBook, order::OrderSpec};

// Default give pre-allocated memory
let mut book = OrderBook::<OrderSpec>::default();

let order = OrderSpec::limit_price(1, OrderSide::Sell, 121, 2);
let result = book.insert_order(&order);
```

```rust
use market_forge::{order::OrderSide, order_book::OrderBook, order::OrderSpec};

let mut book = OrderBook::<OrderSpec>::default();

let order = OrderSpec::market(2, OrderSide::Sell, 5);
let result = book.insert_order(&order);
```

## ✅ TODO

- ✅ Insert Order
- ✅ Cancel Order
- ✅ Replace Order
- ⬜ Recover Order
  - Just use it for recover order price when you missing it
  - It's very slow for high performance matching engine

## ✅ Supported Order Type

✅ Market Order — Executes immediately against the best available prices in the order book. Prioritizes execution speed over price certainty. May experience slippage if liquidity is limited.
✅ Limit Order — Executes only at the specified price or better. Provides price control but does not guarantee execution if the market never reaches the limit price.
✅ Immediate-Or-Cancel (IOC) — Attempts to execute immediately. Any portion that cannot be filled instantly is canceled. Allows partial fills but leaves no remaining order on the book.
✅ Fill-Or-Kill (FOK) — Must be filled completely and immediately. If the entire quantity cannot be executed at once, the whole order is canceled. No partial fills are allowed.

- ⬜ All-Or-None - Must be filled completely and no need immeadiately. If the entire quantity cannot be executed at once, the order will be saved into orderbook and wait until new order can full fill the order. The order type of the All-Or-None was discarded from the project because it's pretty slow and affecting the performance of other transactions

- ❌ Stop Limit
- ❌ Stop Market
- ❌ Stop-Loss
- ❌ Trailing Stop
- ❌ OCO
- ❌ Post-Only
- ❌ Reduce-Only

We not supporting other type not because we won't to do it. But, because we need to makesure our engine is slim and can be embedded into other system.

**For most of unsupported order type it can be created as separated service**

## Supported Order Type on most complete matching engine

| Order Condition     | Crypto Exchanges  | Stock Exchanges | Notes                         |
| ------------------- | ----------------- | --------------- | ----------------------------- |
| Market              | ✅ Always         | ✅ Always       | Instant execution             |
| Limit               | ✅ Always         | ✅ Always       | Goes on order book            |
| Immediate-Or-Cancel | ✅ Supported      | ✅ Supported    | For quick partial fills       |
| Fill-Or-Kill        | ✅ Supported      | ✅ Supported    | For guaranteed full execution |
| All-Or-None         | ❓ Rare           | ✅ Sometimes    | Not always supported          |
| Stop / Stop-Loss    | ✅ Common         | ✅ Common       | Risk management               |
| Trailing Stop       | ✅ Some platforms | ✅ Some brokers | Not in all matching engines   |
| OCO                 | ✅ Often          | ❌ Rare         | More common in crypto         |
| Post-Only           | ✅ Yes            | ❌ Rare         | Ensures maker-only            |
| Reduce-Only         | ✅ Derivatives    | ❌ Rare         | Risk protection in leverage   |

## 🧪 Tests

```bash
cargo test
```
