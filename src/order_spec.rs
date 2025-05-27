use crate::order::{ExecutionCondition, Order, OrderSide, OrderType, Price, Quantity, TimeInForce};

#[derive(Debug, Clone)]
pub struct OrderSpec {
    // Unique identifier for the order
    pub id: u64,

    // Side of the order (Buy or Sell)
    pub order_side: OrderSide,

    // Type of the order (e.g., Market, Limit, Stop)
    pub order_type: OrderType,

    // Price of the order
    pub price: Price,

    // Quantity of the order
    pub quantity: Quantity,

    // Time and execution conditions
    pub time_in_force: TimeInForce,

    // Execution condition for the order
    pub execution_condition: ExecutionCondition,

    // Stop Price for Stop orders
    pub trigger_price: Price,

    // Trail offset for trailing stop orders
    pub trail_offset: Price,
}

impl OrderSpec {
    pub fn new(
        id: u64,
        order_side: OrderSide,
        order_type: OrderType,
        price: Price,
        quantity: Quantity,
    ) -> Self {
        Self {
            id,
            order_side,
            order_type,
            price,
            quantity,
            trigger_price: 0,
            trail_offset: 0,
            time_in_force: TimeInForce::GTC, // Default to GTC
            execution_condition: ExecutionCondition::None, // Default to None
        }
    }
}

impl Order for OrderSpec {
    #[inline(always)]
    fn id(&self) -> u64 {
        self.id
    }

    #[inline(always)]
    fn price(&self) -> Price {
        self.price
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
    fn execution_condition(&self) -> ExecutionCondition {
        self.execution_condition
    }

    #[inline(always)]
    fn set_time_in_force(&mut self, time_in_force: TimeInForce) {
        self.time_in_force = time_in_force;
    }
}
