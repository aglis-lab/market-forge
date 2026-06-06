#[cfg(test)]
mod tests {
    use market_forge::core::{
        order::{OrderSide, TimeInForce},
        order_book::{OrderBook, Trade},
        order_spec::OrderSpec,
    };
    use std::io::Write;
    use std::mem;

    // OrderSpec size: 32 bytes
    // Top price: 118, order price: 111, is match price: false, quantity: 2, is FOK: false
    // Top price: 118, order price: 118, is match price: true, quantity: 15, is FOK: false
    #[test]
    fn order_spec_test() {
        // Initialize the logger so it reads the RUST_LOG env variable``
        let mut builder = env_logger::builder();
        builder.is_test(true);
        builder.format(|buf, record| {
            writeln!(
                buf,
                "{:<5} [{}:{}] - {}",
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        });
        builder.try_init().unwrap();
        builder.filter_level(log::LevelFilter::Debug);

        println!("OrderSpec size: {} bytes", mem::size_of::<OrderSpec>());

        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        book.insert_order(&OrderSpec::limit_price(1, OrderSide::Sell, 121, 12));
        book.insert_order(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        book.insert_order(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));
        book.insert_order(&OrderSpec::limit_price(4, OrderSide::Sell, 118, 5));

        book.insert_order(&OrderSpec::limit_price(5, OrderSide::Buy, 111, 2));
        book.insert_order(&OrderSpec::limit_price(6, OrderSide::Buy, 118, 15));
        book.insert_order(&OrderSpec::limit_price(7, OrderSide::Buy, 122, 10));

        book.insert_order(&OrderSpec::limit_price(8, OrderSide::Sell, 118, 15));

        println!("{}", book);

        assert!(
            book.asks().validate_cache().is_ok(),
            "Ask cache validation failed, {}",
            book.asks().validate_cache().err().unwrap()
        );

        assert!(
            book.bids().validate_cache().is_ok(),
            "Bid cache validation failed"
        );
    }

    #[test]
    fn order_spec_ioc_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        book.insert_order(&OrderSpec::limit_price(1, OrderSide::Sell, 121, 12));
        book.insert_order(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        book.insert_order(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));
        book.insert_order(&OrderSpec::limit_price(4, OrderSide::Sell, 118, 5));

        println!("{}", book);

        // IOC
        book.insert_order(
            &OrderSpec::limit_price(5, OrderSide::Buy, 111, 2).with_time_in_force(TimeInForce::IOC),
        );

        assert_eq!(
            book.bids().len(),
            0,
            "IOC order should not be added to the book"
        );

        println!("{}", book);

        // Matching with top asks
        book.insert_order(
            &OrderSpec::limit_price(6, OrderSide::Buy, 118, 15)
                .with_time_in_force(TimeInForce::IOC),
        );

        // Not matching with top bids
        book.insert_order(
            &OrderSpec::limit_price(6, OrderSide::Sell, 111, 15)
                .with_time_in_force(TimeInForce::IOC),
        );

        println!("{}", book);
        let get_top_asks = {
            book.asks()
                .get_price_level(&book.asks().peek_price().unwrap())
                .iter()
                .next()
                .cloned()
        };

        assert!(get_top_asks.is_some(), "Top ask should exist");
        assert_eq!(
            get_top_asks.unwrap().quantity(),
            10,
            "IOC order should match with the top ask"
        );
        assert_eq!(
            get_top_asks.unwrap().len(),
            2,
            "Asks should have two orders after matching"
        );

        assert!(
            book.asks().validate_cache().is_ok(),
            "Ask cache validation failed"
        );

        assert!(
            book.bids().validate_cache().is_ok(),
            "Bid cache validation failed"
        );

        println!("{}", book)
    }

    #[test]
    fn order_spec_fok_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(10);

        book.insert_order(&OrderSpec::limit_price(1, OrderSide::Sell, 119, 12));
        book.insert_order(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        book.insert_order(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));

        // Not match
        let res = book.insert_order(
            &OrderSpec::limit_price(4, OrderSide::Buy, 120, 23)
                .with_time_in_force(TimeInForce::FOK),
        );
        // Should match None
        assert!(
            res.is_empty(),
            "should not return any match order because full or cancel"
        );

        println!("{}", book);

        // Match with price 119 and 12 quantity
        let res = book.insert_order(
            &OrderSpec::limit_price(4, OrderSide::Buy, 120, 12)
                .with_time_in_force(TimeInForce::FOK),
        );
        // Should match None
        assert_eq!(
            *res,
            vec![Trade {
                match_from_id: 4,
                match_to_id: 1,
                order_side: OrderSide::Buy,
                price: 119,
                quantity: 12,
            }],
            "should not return any match order because full or cancel"
        );

        assert!(
            book.asks().validate_cache().is_ok(),
            "Ask cache validation failed"
        );

        assert!(
            book.bids().validate_cache().is_ok(),
            "Ask cache validation failed"
        );

        println!("{}", book);
    }

    #[test]
    fn order_market_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        book.insert_order(&OrderSpec::limit_price(1, OrderSide::Sell, 121, 12));
        book.insert_order(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        book.insert_order(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));
        book.insert_order(&OrderSpec::limit_price(4, OrderSide::Sell, 118, 5));

        let match_order = book.insert_order(&OrderSpec::market(5, OrderSide::Buy, 2));
        assert_eq!(match_order.len(), 1, "match order should not empty");
        assert_eq!(match_order[0].price, 118, "match order price should be 118");
        assert_eq!(
            match_order[0].quantity, 2,
            "match order quantity should be 2"
        );

        println!("{}", book);

        assert_eq!(
            book.asks().total_quantity(),
            25,
            "total quantity should be 25 after match"
        );

        book.insert_order(&OrderSpec::market(6, OrderSide::Buy, 15));
        assert_eq!(
            book.asks().total_quantity(),
            10,
            "total quantity should be 10 after match"
        );

        book.insert_order(&OrderSpec::market(8, OrderSide::Buy, 15));
        assert_eq!(
            book.asks().total_quantity(),
            0,
            "total quantity should be 0 after match"
        );

        {
            let match_order = book.insert_order(&OrderSpec::market(7, OrderSide::Buy, 10));
            assert!(match_order.is_empty(), "no match order found");
        }

        println!("{}", book);

        if let Some(err) = book.validate_cache().err() {
            panic!("{:?}", err);
        }
    }
}
