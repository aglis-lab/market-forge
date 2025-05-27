pub trait PriceKey: Ord + Copy {}
pub type Price = u128;
pub type Quantity = u128;

pub trait Order: Clone {
    fn id(&self) -> u64;
    fn price(&self) -> Price;
    fn quantity(&self) -> Quantity;
    fn set_quantity(&mut self, new_quantity: Quantity);
    fn order_side(&self) -> OrderSide;

    // Time Force & Execution Condition
    fn time_in_force(&self) -> TimeInForce;

    // Set TimeInForce
    fn set_time_in_force(&mut self, time_in_force: TimeInForce);

    // Copy TimeInForce
    fn with_time_in_force(&mut self, time_in_force: TimeInForce) -> &Self {
        self.set_time_in_force(time_in_force);
        return self;
    }

    fn execution_condition(&self) -> ExecutionCondition;

    // Order Side
    fn is_buy(&self) -> bool {
        return self.order_side().is_buy();
    }

    fn is_sell(&self) -> bool {
        return self.order_side().is_sell();
    }

    // Good Till Cancel
    fn good_till_cancel(&self) -> bool {
        return self.time_in_force() == TimeInForce::GTC;
    }

    // Order Type
    fn immediate_or_cancel(&self) -> bool {
        return self.time_in_force() == TimeInForce::IOC;
    }
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
    GTC, // Good till cancel
    IOC, // Immediate or cancel
    FOK, // Fill or kill
    DAY, // Good for the trading day (optional, often used in stock exchanges)
    GTD, // Good Till Date (optional, usually for advanced systems)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionCondition {
    None, // No condition
    AON,  // All-Or-None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,       // Market order — match now, no price
    Limit,        // Limit order — match at limit price or better
    StopMarket,   // Triggers a Market order when stop price is hit
    StopLimit,    // Triggers a Limit order when stop price is hit
    TrailingStop, // Stop price trails market price
}
