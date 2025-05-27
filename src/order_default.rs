use crate::order::{ExecutionCondition, Order, OrderSide, Price, Quantity, TimeInForce};

#[derive(Debug, Clone)]
pub struct OrderDefault {
    id: u64,
    price: Price,
    quantity: Quantity,
    order_side: OrderSide,
}

impl OrderDefault {
    #[inline(always)]
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
    #[inline(always)]
    fn id(&self) -> u64 {
        return self.id;
    }

    #[inline(always)]
    fn price(&self) -> Price {
        return self.price;
    }

    #[inline(always)]
    fn quantity(&self) -> Quantity {
        return self.quantity;
    }

    #[inline(always)]
    fn order_side(&self) -> OrderSide {
        return self.order_side;
    }

    #[inline(always)]
    fn is_buy(&self) -> bool {
        return self.order_side == OrderSide::Buy;
    }

    #[inline(always)]
    fn is_sell(&self) -> bool {
        return self.order_side == OrderSide::Sell;
    }

    #[inline(always)]
    fn set_quantity(&mut self, new_quantity: Quantity) {
        self.quantity = new_quantity
    }

    #[inline(always)]
    fn time_in_force(&self) -> TimeInForce {
        return TimeInForce::IOC;
    }

    #[inline(always)]
    fn execution_condition(&self) -> ExecutionCondition {
        return ExecutionCondition::None;
    }
}
