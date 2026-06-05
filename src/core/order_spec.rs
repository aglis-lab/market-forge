use rkyv::{Archive, Deserialize, Serialize};

use crate::core::order::{Order, OrderId, OrderSide, OrderType, Price, Quantity, TimeInForce};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
pub struct OrderSpec {
    // Unique identifier for the order
    pub id: OrderId,

    // Price of the order
    pub price: Price,

    // Quantity of the order
    pub quantity: Quantity,

    // Side of the order (Buy or Sell)
    pub order_side: OrderSide,

    // Type of the order (e.g., Market, Limit, Stop)
    pub order_type: OrderType,

    // Time and execution conditions
    pub time_in_force: TimeInForce,
}

impl OrderSpec {
    #[inline(always)]
    pub fn limit_price(
        id: OrderId,
        order_side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> Self {
        Self {
            id,
            order_side,
            price,
            quantity,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC, // Default to GTC
        }
    }

    #[inline(always)]
    pub fn cancel(id: OrderId, order_side: OrderSide, price: Price) -> Self {
        Self {
            id,
            order_side,
            price,
            quantity: 0,                     // NOT BEING USED
            order_type: OrderType::Limit,    // NOT BEING USED
            time_in_force: TimeInForce::GTC, // NOT BEING USED
        }
    }

    #[inline(always)]
    pub fn replace(id: OrderId, order_side: OrderSide, price: Price) -> Self {
        Self {
            id,
            order_side,
            price,
            quantity: 0,                     // NOT BEING USED
            order_type: OrderType::Limit,    // NOT BEING USED
            time_in_force: TimeInForce::GTC, // NOT BEING USED
        }
    }

    #[inline(always)]
    pub fn market(id: OrderId, order_side: OrderSide, quantity: Quantity) -> Self {
        Self {
            id: id,
            order_side,
            quantity,
            price: 0,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC, // Default to GTC
        }
    }
}

impl Order for OrderSpec {
    #[inline(always)]
    fn id(&self) -> OrderId {
        self.id
    }

    #[inline(always)]
    fn price(&self) -> Price {
        self.price
    }

    #[inline(always)]
    fn set_price(&mut self, new_price: Price) {
        self.price = new_price;
    }

    #[inline(always)]
    fn quantity(&self) -> Quantity {
        self.quantity
    }

    #[inline(always)]
    fn set_quantity(&mut self, new_quantity: Quantity) {
        self.quantity = new_quantity;
    }

    #[inline(always)]
    fn order_side(&self) -> OrderSide {
        self.order_side
    }

    #[inline(always)]
    fn order_type(&self) -> OrderType {
        return self.order_type;
    }

    #[inline(always)]
    fn is_buy(&self) -> bool {
        self.order_side.is_buy()
    }

    #[inline(always)]
    fn is_sell(&self) -> bool {
        self.order_side.is_sell()
    }

    #[inline(always)]
    fn time_in_force(&self) -> TimeInForce {
        self.time_in_force
    }

    #[inline(always)]
    fn set_time_in_force(&mut self, time_in_force: TimeInForce) {
        self.time_in_force = time_in_force;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn show_order_spec_size() {
        let size = size_of::<OrderSpec>();
        // Print size; run tests with `-- --nocapture` to see this output.
        println!("OrderSpec size: {} bytes", size);
        assert!(size > 0);
    }
}
