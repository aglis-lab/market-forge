#[cfg(test)]
mod tests {
    use market_forge::{order::OrderSide, order_book::OrderBook, order_default::OrderDefault};

    #[test]
    fn simple_order_test() {
        use std::mem::size_of;
        println!("OrderBook size: {}", size_of::<OrderBook<OrderDefault>>());
        println!("OrderDefault size: {}", size_of::<OrderDefault>());

        let mut book = OrderBook::<OrderDefault>::new(100);

        _ = book.add(&OrderDefault::new(OrderSide::Sell, 3, 121, 12));
        _ = book.add(&OrderDefault::new(OrderSide::Sell, 4, 120, 8));
        _ = book.add(&OrderDefault::new(OrderSide::Sell, 1, 120, 2));
        _ = book.add(&OrderDefault::new(OrderSide::Sell, 2, 118, 5));
        println!("{}\n", book);

        _ = book.add(&OrderDefault::new(OrderSide::Buy, 5, 111, 2));
        _ = book.add(&OrderDefault::new(OrderSide::Buy, 6, 118, 15));
        _ = book.add(&OrderDefault::new(OrderSide::Buy, 7, 122, 10));
        println!("{}\n", book);

        _ = book.add(&OrderDefault::new(OrderSide::Sell, 8, 118, 15));
        println!("{}\n", book)
    }
}
