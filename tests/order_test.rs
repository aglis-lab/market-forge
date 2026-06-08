#[cfg(test)]
mod tests {
    use std::mem;

    use market_forge::{
        core::{
            error::OrderError,
            order::OrderSpec,
            order::{OrderSide, TimeInForce},
            order_book::{self, OrderBook, Trade},
        },
        utils::ReverseOrd,
    };

    const SYMBOL: u32 = 1;

    #[test]
    fn order_test() {
        println!("OrderSpec size: {} bytes", mem::size_of::<OrderSpec>());

        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 1, OrderSide::Sell, 121, 12));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 2, OrderSide::Sell, 120, 8));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 3, OrderSide::Sell, 120, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 4, OrderSide::Sell, 118, 5));

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 5, OrderSide::Buy, 111, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 6, OrderSide::Buy, 118, 15));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 7, OrderSide::Buy, 122, 10));

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 8, OrderSide::Sell, 118, 15));

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
    fn order_ioc_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 1, OrderSide::Sell, 121, 12));
        println!("{}", book);
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 2, OrderSide::Sell, 120, 8));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 3, OrderSide::Sell, 120, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 4, OrderSide::Sell, 118, 5));
        println!("{}", book);

        _ = book.insert_order(
            &OrderSpec::limit_price(SYMBOL, 5, OrderSide::Buy, 111, 2)
                .with_time_in_force(TimeInForce::IOC),
        );
        assert_eq!(
            book.bids().len(),
            0,
            "IOC order should not be added to the book"
        );

        println!("{}", book);
        // Matching with top asks
        _ = book.insert_order(
            &OrderSpec::limit_price(SYMBOL, 6, OrderSide::Buy, 118, 15)
                .with_time_in_force(TimeInForce::IOC),
        );

        // Not matching with top bids
        _ = book.insert_order(
            &OrderSpec::limit_price(SYMBOL, 6, OrderSide::Sell, 111, 15)
                .with_time_in_force(TimeInForce::IOC),
        );

        println!("{}", book);
        let get_top_asks = book
            .asks()
            .get_price_level(&book.asks().peek_price().unwrap());

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
            "Ask cache validation failed"
        );

        println!("{}", book)
    }

    #[test]
    fn order_fok_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(10);

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 1, OrderSide::Sell, 119, 12));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 2, OrderSide::Sell, 120, 8));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 3, OrderSide::Sell, 120, 2));

        // Not match
        let res = book.insert_order(
            &OrderSpec::limit_price(SYMBOL, 4, OrderSide::Buy, 120, 23)
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
            &OrderSpec::limit_price(SYMBOL, 4, OrderSide::Buy, 120, 12)
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

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 1, OrderSide::Sell, 121, 12));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 2, OrderSide::Sell, 120, 8));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 3, OrderSide::Sell, 120, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 4, OrderSide::Sell, 118, 5));

        let match_order = book.insert_order(&OrderSpec::market(SYMBOL, 5, OrderSide::Buy, 2));
        assert_eq!(match_order.len(), 1, "match order should not empty");
        assert_eq!(match_order[0].price, 118, "match order price should be 118");
        assert_eq!(
            match_order[0].quantity, 2,
            "match order quantity should be 2"
        );

        assert_eq!(
            book.asks()
                .get_price_level(&book.asks().peek_price().unwrap())
                .unwrap()
                .quantity(),
            3,
            "quantity should be 3 after match"
        );

        assert_eq!(
            book.asks().total_quantity(),
            25,
            "total quantity should be 25 after match"
        );

        _ = book.insert_order(&OrderSpec::market(SYMBOL, 6, OrderSide::Buy, 15));
        assert_eq!(
            book.asks()
                .get_price_level(&book.asks().peek_price().unwrap())
                .unwrap()
                .quantity(),
            10,
            "quantity should be 10 after match"
        );

        assert_eq!(
            book.asks().total_quantity(),
            10,
            "total quantity should be 10 after match"
        );

        _ = book.insert_order(&OrderSpec::market(SYMBOL, 8, OrderSide::Buy, 15));
        assert_eq!(
            book.asks().total_quantity(),
            0,
            "total quantity should be 0 after match"
        );

        {
            let match_order = book.insert_order(&OrderSpec::market(SYMBOL, 7, OrderSide::Buy, 10));
            assert!(match_order.is_empty(), "no match order found");
        }

        println!("{}", book);

        if let Some(err) = book.validate_cache().err() {
            panic!("{:?}", err);
        }
    }

    #[test]
    fn order_cancel_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 1, OrderSide::Sell, 121, 12));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 2, OrderSide::Sell, 120, 8));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 3, OrderSide::Sell, 120, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 4, OrderSide::Sell, 118, 5));

        assert!(
            book.cancel_order(5).is_err(),
            "Canceling order id 5 should return an error"
        );

        assert!(
            book.cancel_order(1).is_ok(),
            "Canceling order id 1 should return an ok"
        );

        assert_eq!(
            book.asks().len(),
            2,
            "orders should only 2 exist with 2 order and 1 order respectively"
        );

        println!("{}", book);

        if let Some(err) = book.validate_cache().err() {
            panic!("{:?}", err);
        }
    }

    #[test]
    fn order_replace_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 1, OrderSide::Sell, 121, 12));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 2, OrderSide::Sell, 120, 8));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 3, OrderSide::Sell, 120, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 4, OrderSide::Sell, 118, 5));

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 5, OrderSide::Buy, 115, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 6, OrderSide::Buy, 116, 15));

        // Replace Order Id 5 Quantity from 2 to 0
        let should_err = book.replace_order(order_book::ReplaceOrder {
            order_id: 5,
            new_price: 0,
            quantity_delta: -2,
        });

        assert_eq!(
            should_err.err(),
            Some(OrderError::InvalidQuantityDelta),
            "Invalid quantity delta should return error"
        );
        println!("{}", book);

        // Check validation
        if let Some(err) = book.validate_cache().err() {
            panic!("{:?}", err);
        }
    }

    #[test]
    fn order_replace_success_test() {
        let mut book = OrderBook::<OrderSpec>::with_capacity(100);

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 1, OrderSide::Sell, 121, 12));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 2, OrderSide::Sell, 120, 8));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 3, OrderSide::Sell, 120, 2));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 4, OrderSide::Sell, 118, 5));

        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 5, OrderSide::Buy, 115, 10));
        _ = book.insert_order(&OrderSpec::limit_price(SYMBOL, 6, OrderSide::Buy, 116, 15));

        // Replace Order Id 5 Quantity from 10 to 8
        let _ = book.replace_order(order_book::ReplaceOrder {
            order_id: 5,
            new_price: 0,
            quantity_delta: -2,
        });

        let should_matches = book
            .replace_order(order_book::ReplaceOrder {
                order_id: 6,
                new_price: 118,
                quantity_delta: 0,
            })
            .unwrap();

        assert_eq!(
            should_matches[0].quantity, 5,
            "Replace order should match with with 5 quantity"
        );

        assert_eq!(
            should_matches[0].match_to_id, 4,
            "Replace order should match with order id 4"
        );

        assert_eq!(
            *book.asks().peek_price().unwrap(),
            120,
            "peek price level quantity should be 120 after replace order"
        );

        assert_eq!(
            book.asks()
                .get_price_level(&book.asks().peek_price().unwrap())
                .unwrap()
                .quantity(),
            10,
            "peek price level quantity should be 10 after replace order"
        );

        assert_eq!(
            book.bids().peek_price().unwrap().0,
            118,
            "peek price level quantity should be 118 after replace order"
        );

        assert_eq!(
            book.bids()
                .get_price_level(&ReverseOrd::new(book.bids().peek_price().unwrap().0))
                .unwrap()
                .quantity(),
            10,
            "peek price level quantity should be 10 after replace order"
        );

        println!("{}", book);

        // Check validation
        if let Some(err) = book.validate_cache().err() {
            panic!("{:?}", err);
        }
    }
}
