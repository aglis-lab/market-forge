pub type Price = u64;
pub type Quantity = u64;
pub type OrderId = u32;

pub trait Order: Clone {
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

    // Execution Condition
    fn execution_condition(&self) -> ExecutionCondition;

    // Set TimeInForce
    fn set_time_in_force(&mut self, time_in_force: TimeInForce);

    // Copy TimeInForce
    #[inline(always)]
    fn with_time_in_force(&mut self, time_in_force: TimeInForce) -> &Self {
        self.set_time_in_force(time_in_force);
        return self;
    }

    #[inline(always)]
    fn with_price(&mut self, new_price: Price) -> &Self {
        self.set_price(new_price);
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
        self.order_type() == OrderType::Market
    }

    #[inline(always)]
    fn is_limit_price(&self) -> bool {
        self.order_type() == OrderType::Limit
    }

    // Good Till Cancel
    #[inline(always)]
    fn good_till_cancel(&self) -> bool {
        return self.time_in_force() == TimeInForce::GTC;
    }

    // should not lived at slab allocator because we discard the order from the system immediately
    #[inline(always)]
    fn is_ephemeral_order(&self) -> bool {
        return self.is_immediate_or_cancel() || self.is_fill_or_kill();
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

impl OrderType {
    #[inline(always)]
    pub fn is_market(&self) -> bool {
        *self == OrderType::Market
    }

    #[inline(always)]
    pub fn is_limit(&self) -> bool {
        *self == OrderType::Limit
    }

    #[inline(always)]
    pub fn is_stop_market(&self) -> bool {
        *self == OrderType::StopMarket
    }

    #[inline(always)]
    pub fn is_stop_limit(&self) -> bool {
        *self == OrderType::StopLimit
    }

    #[inline(always)]
    pub fn is_trailing_stop(&self) -> bool {
        *self == OrderType::TrailingStop
    }
}
