use crate::order_match::OrderMatch;

pub trait PriceKey: Ord + Copy {}
pub type Price = u128;
pub type Quantity = u128;

#[derive(PartialEq, Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

pub trait Order: Clone {
    fn id(&self) -> u64;
    fn price(&self) -> Price;
    fn quantity(&self) -> Quantity;
    fn set_quantity(&mut self, new_quantity: Quantity);
    fn order_side(&self) -> &OrderSide;

    fn is_buy(&self) -> bool;
    fn is_sell(&self) -> bool;
    fn is_match_price(&self, other_price: Price) -> bool;

    // Every match we produce 2 match order
    // Match order have 2 type partial order and full order
    // We also return new order with quantity subtract from other quantity
    fn match_order<T: Order>(&mut self, other: &mut T) -> OrderMatch;
}

impl Clone for OrderSide {
    fn clone(&self) -> Self {
        match self {
            Self::Buy => Self::Buy,
            Self::Sell => Self::Sell,
        }
    }
}
