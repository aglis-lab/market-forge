use crate::core::order::{OrderId, OrderSide, Price, Quantity};

#[derive(Debug, PartialEq)]
pub struct OrderMatch {
    pub order_side: OrderSide,
    pub price: Price,
    pub quantity: Quantity,

    pub match_from_id: OrderId,
    pub match_to_id: OrderId,
}
