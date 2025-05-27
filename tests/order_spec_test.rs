#[cfg(test)]
mod tests {
    use market_forge::{
        order::{Order, OrderSide, OrderType, TimeInForce},
        order_book::OrderBook,
        order_spec::OrderSpec,
    };

    #[test]
    fn order_spec_test() {
        let mut book = OrderBook::<OrderSpec>::new(100);

        _ = book.add(&OrderSpec::new(
            1,
            OrderSide::Sell,
            OrderType::Market,
            121,
            12,
        ));
        _ = book.add(&OrderSpec::new(
            2,
            OrderSide::Sell,
            OrderType::Market,
            120,
            8,
        ));
        _ = book.add(&OrderSpec::new(
            3,
            OrderSide::Sell,
            OrderType::Market,
            120,
            2,
        ));
        _ = book.add(&OrderSpec::new(
            4,
            OrderSide::Sell,
            OrderType::Market,
            118,
            5,
        ));

        _ = book.add(&OrderSpec::new(
            5,
            OrderSide::Buy,
            OrderType::Market,
            111,
            2,
        ));
        _ = book.add(&OrderSpec::new(
            6,
            OrderSide::Buy,
            OrderType::Market,
            118,
            15,
        ));
        _ = book.add(&OrderSpec::new(
            7,
            OrderSide::Buy,
            OrderType::Market,
            122,
            10,
        ));

        _ = book.add(&OrderSpec::new(
            8,
            OrderSide::Sell,
            OrderType::Market,
            118,
            15,
        ));
    }

    #[test]
    fn order_spec_ioc_test() {
        let mut book = OrderBook::<OrderSpec>::new(100);

        _ = book.add(&OrderSpec::new(
            1,
            OrderSide::Sell,
            OrderType::Market,
            121,
            12,
        ));
        _ = book.add(&OrderSpec::new(
            2,
            OrderSide::Sell,
            OrderType::Market,
            120,
            8,
        ));
        _ = book.add(&OrderSpec::new(
            3,
            OrderSide::Sell,
            OrderType::Market,
            120,
            2,
        ));
        _ = book.add(&OrderSpec::new(
            4,
            OrderSide::Sell,
            OrderType::Market,
            118,
            5,
        ));

        _ = book.add(
            &OrderSpec::new(5, OrderSide::Buy, OrderType::Market, 111, 2)
                .with_time_in_force(TimeInForce::IOC),
        );
        assert_eq!(
            book.bids().len(),
            0,
            "IOC order should not be added to the book"
        );

        println!("{}", book);
        // Matching with top asks
        _ = book.add(
            &OrderSpec::new(6, OrderSide::Buy, OrderType::Market, 118, 15)
                .with_time_in_force(TimeInForce::IOC),
        );

        // Not matching with top bids
        _ = book.add(
            &OrderSpec::new(6, OrderSide::Sell, OrderType::Market, 111, 15)
                .with_time_in_force(TimeInForce::IOC),
        );

        println!("{}", book);
        let get_top_asks = {
            book.asks()
                .get_orders(&book.asks().peek().unwrap())
                .iter()
                .next()
                .cloned()
        };

        assert!(get_top_asks.is_some(), "Top ask should exist");
        assert_eq!(
            get_top_asks.unwrap().total_quantity(),
            10,
            "IOC order should match with the top ask"
        );
        assert_eq!(
            get_top_asks.unwrap().len(),
            2,
            "Asks should have two orders after matching"
        );

        println!("{}", book)
    }
}
