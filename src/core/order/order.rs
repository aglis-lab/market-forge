use std::fmt::Debug;

use rkyv::{Archive, Deserialize, Serialize};

pub type Price = u64;
pub type Quantity = u64;
pub type SignQuantity = i64;
pub type OrderId = u64;

pub trait Order: Clone + Debug + PartialEq {
    // id
    fn id(&self) -> OrderId;

    // Price of ther order
    fn price(&self) -> Price;
    fn set_price(&mut self, new_price: Price);

    // Quantity
    fn quantity(&self) -> Quantity;
    fn set_quantity(&mut self, new_quantity: Quantity);

    // Order Side
    fn order_side(&self) -> OrderSide;

    // Order Type
    fn order_type(&self) -> OrderType;

    // Time Force & Execution Condition
    fn time_in_force(&self) -> TimeInForce;

    // Set TimeInForce
    fn set_time_in_force(&mut self, time_in_force: TimeInForce);

    // Order Side
    fn is_buy(&self) -> bool;
    fn is_sell(&self) -> bool;

    // Order Type
    fn is_market(&self) -> bool;
    fn is_limit_price(&self) -> bool;

    // Time in Force
    fn good_till_cancel(&self) -> bool;
    fn is_immediate_or_cancel(&self) -> bool;
    fn is_fill_or_kill(&self) -> bool;
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
pub enum OrderSide {
    Buy,  // Buy order
    Sell, // Sell order
}

impl OrderSide {
    #[inline(always)]
    pub fn is_buy(self) -> bool {
        return self == OrderSide::Buy;
    }

    #[inline(always)]
    pub fn is_sell(self) -> bool {
        return self == OrderSide::Sell;
    }
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
pub enum TimeInForce {
    GTC, // Good till cancel
    IOC, // Immediate or cancel
    FOK, // Fill or kill
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
pub enum OrderType {
    Market, // Market order — match now, no price
    Limit,  // Limit order — match at limit price or better
}
