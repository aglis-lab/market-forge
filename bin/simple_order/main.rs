use market_forge::{
    order::{self, OrderSide},
    order_book,
};

fn main() {
    let mut book = order_book::OrderBook::new(None);

    book.add(&order::Order::new(OrderSide::Sell, 1, 120, 11));
    book.add(&order::Order::new(OrderSide::Sell, 1, 121, 5));
    book.add(&order::Order::new(OrderSide::Sell, 1, 121, 12));
    book.add(&order::Order::new(OrderSide::Sell, 1, 123, 8));

    book.add(&order::Order::new(OrderSide::Buy, 1, 111, 2));
    book.add(&order::Order::new(OrderSide::Buy, 1, 118, 15));
    book.add(&order::Order::new(OrderSide::Buy, 1, 122, 4));

    println!("{}", book)
}
