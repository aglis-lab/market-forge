use std::fmt::Debug;

use rkyv::{Archive, Deserialize, Serialize};

pub type Price = u64;
pub type Quantity = u64;
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

    // Copy TimeInForce
    #[inline(always)]
    fn with_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.set_time_in_force(time_in_force);
        return self;
    }

    #[inline(always)]
    fn with_price(mut self, new_price: Price) -> Self {
        self.set_price(new_price);
        return self;
    }

    #[inline(always)]
    fn with_quantity(mut self, new_quantity: Quantity) -> Self {
        self.set_quantity(new_quantity);
        return self;
    }

    // Order Side
    #[inline(always)]
    fn is_buy(&self) -> bool {
        return self.order_side().is_buy();
    }

    #[inline(always)]
    fn is_sell(&self) -> bool {
        return self.order_side().is_sell();
    }

    #[inline(always)]
    fn is_market(&self) -> bool {
        self.order_type().is_market()
    }

    #[inline(always)]
    fn is_limit_price(&self) -> bool {
        self.order_type().is_limit()
    }

    // Good Till Cancel
    #[inline(always)]
    fn good_till_cancel(&self) -> bool {
        return self.time_in_force() == TimeInForce::GTC;
    }

    // is immediate or cancel
    #[inline(always)]
    fn is_immediate_or_cancel(&self) -> bool {
        return self.time_in_force() == TimeInForce::IOC;
    }

    // is fill or kill
    #[inline(always)]
    fn is_fill_or_kill(&self) -> bool {
        return self.time_in_force() == TimeInForce::FOK;
    }
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

impl OrderType {
    #[inline(always)]
    pub fn is_market(&self) -> bool {
        *self == OrderType::Market
    }

    #[inline(always)]
    pub fn is_limit(&self) -> bool {
        *self == OrderType::Limit
    }
}
