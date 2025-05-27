#[cfg(test)]
mod tests {
    use market_forge::{
        order::{ExecutionCondition, OrderSide, OrderType, TimeInForce},
        order_book::OrderBook,
        order_spec::OrderSpec,
    };

    #[test]
    fn order_spec_test() {
        use std::mem::size_of;
        println!("OrderSpec size: {} bytes", size_of::<OrderSpec>());

        let mut book = OrderBook::<OrderSpec>::new(100);

        _ = book.add(&OrderSpec {
            id: 1,
            price: 121,
            quantity: 12,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        _ = book.add(&OrderSpec {
            id: 2,
            price: 120,
            quantity: 8,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        _ = book.add(&OrderSpec {
            id: 3,
            price: 120,
            quantity: 2,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        _ = book.add(&OrderSpec {
            id: 4,
            price: 118,
            quantity: 5,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        println!("{}\n", book);

        _ = book.add(&OrderSpec {
            id: 5,
            price: 111,
            quantity: 2,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        _ = book.add(&OrderSpec {
            id: 6,
            price: 118,
            quantity: 15,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        _ = book.add(&OrderSpec {
            id: 7,
            price: 122,
            quantity: 10,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        println!("{}\n", book);

        _ = book.add(&OrderSpec {
            id: 8,
            price: 118,
            quantity: 15,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::None,
            execution_condition: ExecutionCondition::None,
            trigger_price: 0,
            trail_offset: 0,
        });
        println!("{}\n", book)
    }
}
