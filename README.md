# Market Forge

**Market Forge** is a high-performance, Rust-based matching engine. This project aims to provide a modern, memory-safe, and efficient core for building financial exchanges and trading platforms.

## 🚀 Why Market Forge?

- ✨ Written in **Rust** for safety, concurrency, and performance
- 🔁 Efficient **limit order book** matching logic
- 🧩 Modular and easy to integrate into trading systems
- ♻️ Seems clean for me, hahahh

## 📦 Features

- Price-time priority matching
- Supports limit, market, and cancel orders
- Unit-tested and performance-optimized core

## 📚 Example

### Order Spec Implementation

You can implement your own Order trait but the default should be already sufficient enough for most usecases.

```rust
use market_forge::{order::OrderSide, order_book::OrderBook, order_spec::OrderSpec};

let mut book = OrderBook::<OrderSpec>::new();

let order = OrderSpec::limit_price(1, OrderSide::Sell, 121, 2);
let result = book.add(&order);
```

```rust
use market_forge::{order::OrderSide, order_book::OrderBook, order_spec::OrderSpec};

let mut book = OrderBook::<OrderSpec>::new();

let order = OrderSpec::market(2, OrderSide::Sell, 5);
let result = book.add(&order);
```

## ✅ TODO

- ✅ Insert Order
- ✅ Cancel Order
- ✅ Replace Order
- ⬜ Recover Order
  - Just use it for recover order price when you missing it
  - It's very slow for high performance matching engine

## ✅ Supported Order Type

- ✅ Market
- ✅ Limit
- ✅ Immediate-Or-Cancel
- ✅ Fill-Or-Kill
- ❌ All-Or-None
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
