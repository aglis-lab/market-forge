use crate::order::{OrderSide, Price, Quantity};

#[derive(Debug)]
pub struct OrderMatch {
    pub order_side: OrderSide,
    pub price: Price,
    pub quantity: Quantity,

    pub match_from_id: u64,
    pub match_to_id: u64,
}
