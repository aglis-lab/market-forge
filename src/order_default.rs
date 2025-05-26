use std::cmp;

use crate::{
    order::{ExecutionCondition, Order, OrderSide, Price, Quantity, TimeInForce},
    order_match::OrderMatch,
};

#[derive(Debug, Clone)]
pub struct OrderDefault {
    id: u64,
    price: Price,
    quantity: Quantity,
    order_side: OrderSide,
}

impl OrderDefault {
    pub fn new(order_side: OrderSide, id: u64, price: Price, quantity: Quantity) -> Self {
        return OrderDefault {
            id,
            price,
            quantity,
            order_side,
        };
    }
}

impl Order for OrderDefault {
    fn id(&self) -> u64 {
        return self.id;
    }

    fn price(&self) -> Price {
        return self.price;
    }

    fn quantity(&self) -> Quantity {
        return self.quantity;
    }

    fn order_side(&self) -> OrderSide {
        return self.order_side;
    }

    fn is_buy(&self) -> bool {
        return self.order_side == OrderSide::Buy;
    }

    fn is_sell(&self) -> bool {
        return self.order_side == OrderSide::Sell;
    }

    fn is_match_price(&self, other_price: &Price) -> bool {
        if self.is_buy() && *other_price <= self.price() {
            return true;
        }

        if self.is_sell() && *other_price >= self.price() {
            return true;
        }

        return false;
    }

    fn set_quantity(&mut self, new_quantity: Quantity) {
        self.quantity = new_quantity
    }

    fn match_order<T: Order>(&mut self, result: &mut T) -> OrderMatch {
        let min_quantity = cmp::min(self.quantity, result.quantity());

        // Set Quantity both side
        result.set_quantity(result.quantity() - min_quantity);
        self.quantity -= min_quantity;

        // Set Order Match
        let order_match = OrderMatch {
            order_side: result.order_side().clone(), // You may need to fill these fields appropriately
            price: self.price(),
            quantity: min_quantity,
            match_from_id: result.id(),
            match_to_id: self.id(),
        };

        return order_match;
    }

    fn time_in_force(&self) -> TimeInForce {
        return TimeInForce::IOC;
    }

    fn execution_condition(&self) -> ExecutionCondition {
        return ExecutionCondition::None;
    }
}
