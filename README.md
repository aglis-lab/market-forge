# Market Forge

**Market Forge** is a high-performance, Rust-based matching engine inspired by [Liquibook](https://github.com/objectcomputing/liquibook), originally implemented in C++. This project aims to provide a modern, memory-safe, and efficient core for building financial exchanges and trading platforms.

## 🚀 Why Market Forge?

- ✨ Written in **Rust** for safety, concurrency, and performance
- 🔁 Efficient **limit order book** matching logic
- 🧩 Modular and easy to integrate into trading systems
- ♻️ A clean rewrite of the proven Liquibook engine

## 📦 Features

- Price-time priority matching
- Supports limit, market, and cancel orders
- Multiple symbol (book) management
- Pluggable event listeners for trades and book updates
- Unit-tested and performance-optimized core

## 📚 Example

```rust
use market_forge::{OrderBook, Order, Side};

let mut book = OrderBook::new("BTC-USD");

let order = Order::new("order-1", Side::Buy, 100, 50000);
book.process(order);
```

## 🧱 Architecture

order_book: Core matching engine for a single instrument

types: Reusable primitives (Order, Side, Price, etc.)

event: Listener interfaces for fills, book changes, and more

## ✅ TODO

- [ ] Market orders
- [ ] IOC / FOK order support
- [ ] Self-trade prevention
- [ ] Order persistence layer

## 🧪 Tests

```bash
cargo test
```

## 📄 License

Market Forge is open-source under the MIT License.

## 🙌 Credits

Inspired by Liquibook by Object Computing, Inc.
