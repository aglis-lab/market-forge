#[cfg(test)]
mod tests {
    use market_forge::{order::OrderSide, order_book::OrderBook, order_default::OrderDefault};

    #[test]
    fn simple_order_test() {
        use std::mem::size_of;
        println!("OrderBook size: {}", size_of::<OrderBook<OrderDefault>>());

        let mut book = OrderBook::<OrderDefault>::new();

        let result = book.add(&OrderDefault::new(OrderSide::Sell, 1, 120, 2));
        println!("{:?}", result);

        let result = book.add(&OrderDefault::new(OrderSide::Sell, 2, 121, 5));
        println!("{:?}", result);

        let result = book.add(&OrderDefault::new(OrderSide::Sell, 3, 121, 12));
        println!("{:?}", result);

        let result = book.add(&OrderDefault::new(OrderSide::Sell, 4, 123, 8));
        println!("{:?}", result);

        let result = book.add(&OrderDefault::new(OrderSide::Buy, 5, 111, 2));
        println!("{:?}", result);

        let result = book.add(&OrderDefault::new(OrderSide::Buy, 6, 118, 15));
        println!("{:?}", result);

        let result = book.add(&OrderDefault::new(OrderSide::Buy, 7, 122, 10));
        println!("{:?}", result);

        let result = book.add(&OrderDefault::new(OrderSide::Sell, 8, 118, 15));
        println!("{:?}", result);

        println!("{}", book)
    }
}
