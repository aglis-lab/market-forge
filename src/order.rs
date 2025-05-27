pub trait PriceKey: Ord + Copy {}
pub type Price = u128;
pub type Quantity = u128;

pub trait Order: Clone {
    fn id(&self) -> u64;
    fn price(&self) -> Price;
    fn quantity(&self) -> Quantity;
    fn set_quantity(&mut self, new_quantity: Quantity);
    fn order_side(&self) -> OrderSide;

    fn is_buy(&self) -> bool;
    fn is_sell(&self) -> bool;

    // Time Force & Execution Condition
    fn time_in_force(&self) -> TimeInForce;
    fn execution_condition(&self) -> ExecutionCondition;

    // Order Type
    // fn should_trigger(&mut self, last_price: f64) -> bool;
    // fn update_price(&mut self, last_price: f64); // For trailing stop
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    None, // No time in force
    GTC,  // Good till cancel
    IOC,  // Immediate or cancel
    FOK,  // Fill or kill
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionCondition {
    None, // No condition
    AON,  // All-Or-None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,       // Market order
    Limit,        // Limit order
    Stop,         // Stop order
    TrailingStop, // Trailing stop order
}
