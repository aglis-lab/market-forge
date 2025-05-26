use crate::order::{ExecutionCondition, OrderSide, OrderType, Price, Quantity, TimeInForce};

pub struct OrderSpec {
    pub id: u64,
    pub side: OrderSide, // Buy or Sell
    pub order_type: OrderType,
    pub price: Option<Price>, // Limit price or trigger price
    pub qty: Quantity,
    pub remaining_qty: Quantity, // Remaining quantity to be filled

    // Time and execution conditions
    pub time_in_force: TimeInForce,
    pub execution_condition: ExecutionCondition,

    // Stop / Trailing Stop
    pub trigger_price: Option<Price>,
    pub trail_offset: Option<Price>,

    // Internal state
    pub active: bool,
}

// impl Order for OrderSpec {
//     fn new(
//         id: u64,
//         side: Side,
//         order_type: OrderType,
//         price: Option<f64>,
//         qty: f64,
//         time_in_force: TimeInForce,
//         execution_condition: ExecutionCondition,
//     ) -> Self {
//         Self {
//             id,
//             side,
//             order_type,
//             price,
//             qty,
//             remaining_qty: qty, // Initially, remaining quantity is the same as the original quantity
//             time_in_force,
//             execution_condition,
//             trigger_price: None,
//             trail_offset: None,
//             active: true, // Order is active when created
//         }
//     }

//     // Additional methods can be added here for order management
// }
