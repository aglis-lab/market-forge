#[cfg(test)]
mod tests {
    use market_forge::{
        order::{Order, OrderSide, TimeInForce},
        order_book::OrderBook,
        order_match::OrderMatch,
        order_spec::OrderSpec,
    };

    #[test]
    fn order_spec_test() {
        let mut book = OrderBook::<OrderSpec>::new(100);

        _ = book.add(&OrderSpec::limit_price(1, OrderSide::Sell, 121, 12));
        _ = book.add(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        _ = book.add(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));
        _ = book.add(&OrderSpec::limit_price(4, OrderSide::Sell, 118, 5));

        _ = book.add(&OrderSpec::limit_price(5, OrderSide::Buy, 111, 2));
        _ = book.add(&OrderSpec::limit_price(6, OrderSide::Buy, 118, 15));
        _ = book.add(&OrderSpec::limit_price(7, OrderSide::Buy, 122, 10));

        _ = book.add(&OrderSpec::limit_price(8, OrderSide::Sell, 118, 15));

        println!("{}", book);

        assert!(
            book.asks().validate_cache().is_ok(),
            "Ask cache validation failed"
        );

        assert!(
            book.bids().validate_cache().is_ok(),
            "Ask cache validation failed"
        );
    }

    #[test]
    fn order_spec_ioc_test() {
        let mut book = OrderBook::<OrderSpec>::new(100);

        _ = book.add(&OrderSpec::limit_price(1, OrderSide::Sell, 121, 12));
        println!("{}", book);
        _ = book.add(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        _ = book.add(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));
        _ = book.add(&OrderSpec::limit_price(4, OrderSide::Sell, 118, 5));
        println!("{}", book);

        _ = book.add(
            &OrderSpec::limit_price(5, OrderSide::Buy, 111, 2).with_time_in_force(TimeInForce::IOC),
        );
        assert_eq!(
            book.bids().len(),
            0,
            "IOC order should not be added to the book"
        );

        println!("{}", book);
        // Matching with top asks
        _ = book.add(
            &OrderSpec::limit_price(6, OrderSide::Buy, 118, 15)
                .with_time_in_force(TimeInForce::IOC),
        );

        // Not matching with top bids
        _ = book.add(
            &OrderSpec::limit_price(6, OrderSide::Sell, 111, 15)
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
            get_top_asks.unwrap().orders_quantity(),
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
            "Ask cache validation failed"
        );

        println!("{}", book)
    }

    #[test]
    fn order_spec_fok_test() {
        let mut book = OrderBook::<OrderSpec>::new(10);

        _ = book.add(&OrderSpec::limit_price(1, OrderSide::Sell, 119, 12));
        _ = book.add(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        _ = book.add(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));

        // Not match
        let res = book.add(
            &OrderSpec::limit_price(4, OrderSide::Buy, 120, 23)
                .with_time_in_force(TimeInForce::FOK),
        );
        // Should match None
        assert_eq!(
            res, None,
            "should not return any match order because full or cancel"
        );

        println!("{}", book);

        // Match with price 119 and 12 quantity
        let res = book.add(
            &OrderSpec::limit_price(4, OrderSide::Buy, 120, 12)
                .with_time_in_force(TimeInForce::FOK),
        );
        // Should match None
        assert_eq!(
            res,
            Some(vec![OrderMatch {
                match_from_id: 4,
                match_to_id: 1,
                order_side: OrderSide::Buy,
                price: 119,
                quantity: 12,
            }]),
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
        let mut book = OrderBook::<OrderSpec>::new(100);

        _ = book.add(&OrderSpec::limit_price(1, OrderSide::Sell, 121, 12));
        _ = book.add(&OrderSpec::limit_price(2, OrderSide::Sell, 120, 8));
        _ = book.add(&OrderSpec::limit_price(3, OrderSide::Sell, 120, 2));
        _ = book.add(&OrderSpec::limit_price(4, OrderSide::Sell, 118, 5));

        let match_order = book.add(&OrderSpec::market(5, OrderSide::Buy, 2)).unwrap();
        assert_eq!(match_order.len(), 1, "match order should not empty");
        assert_eq!(match_order[0].price, 118, "match order price should be 118");
        assert_eq!(
            match_order[0].quantity, 2,
            "match order quantity should be 2"
        );

        assert_eq!(
            book.asks()
                .orders()
                .iter()
                .next()
                .unwrap()
                .1
                .orders_quantity(),
            3,
            "quantity should be 3 after match"
        );

        assert_eq!(
            book.asks().total_quantity(),
            25,
            "total quantity should be 25 after match"
        );

        _ = book.add(&OrderSpec::market(6, OrderSide::Buy, 15));
        assert_eq!(
            book.asks()
                .orders()
                .iter()
                .next()
                .unwrap()
                .1
                .orders_quantity(),
            10,
            "quantity should be 10 after match"
        );

        assert_eq!(
            book.asks().total_quantity(),
            10,
            "total quantity should be 10 after match"
        );

        _ = book.add(&OrderSpec::market(8, OrderSide::Buy, 15));
        assert_eq!(
            book.asks().total_quantity(),
            0,
            "total quantity should be 0 after match"
        );

        {
            let match_order = book.add(&OrderSpec::market(7, OrderSide::Buy, 10));
            assert!(match_order == None, "no match order found");
        }

        println!("{}", book);

        assert!(
            book.asks().validate_cache().is_ok(),
            "Ask cache validation failed"
        );

        assert!(
            book.bids().validate_cache().is_ok(),
            "Ask cache validation failed"
        );
    }
}
