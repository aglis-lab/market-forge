# Market Forge

**Market Forge** is a high-performance, Rust-based matching engine. This project aims to provide a modern, memory-safe, and efficient core for building financial exchanges and trading platforms.

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

### Order Default Implementation

```rust
use market_forge::{order::OrderSide, order_book::OrderBook, order_default::OrderDefault};

let mut book = OrderBook::<OrderDefault>::new();

let order = OrderDefault::new(OrderSide::Sell, 2, 121, 5);
let result = book.add(&order);
```

### Order Spec Implementation

```rust
use market_forge::{order::OrderSide, order_book::OrderBook, order_spec::OrderSpec};

let mut book = OrderBook::<OrderSpec>::new();

let order = OrderSpec::new(OrderSide::Sell, 2, 121, 5);
let result = book.add(&order);
```

## 🧱 Architecture

order_book: Core matching engine for a single instrument

types: Reusable primitives (Order, Side, Price, etc.)

event: Listener interfaces for fills, book changes, and more

## ✅ TODO

- [x] Slab Allocator (using hashmap and binaryheap for order book)
- [ ] Market orders
- [ ] IOC / FOK order support
- [ ] Self-trade prevention
- [ ] Order persistence layer

## 🧪 Tests

```bash
cargo test
```
