use crate::core::order::{OrderId, OrderSide, Price, Quantity};

#[derive(Debug, PartialEq)]
pub struct Trade {
    pub price: Price,
    pub quantity: Quantity,

    pub match_from_id: OrderId,
    pub match_to_id: OrderId,

    pub order_side: OrderSide,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn show_trade_size() {
        let size = size_of::<Trade>();
        // Print size; run tests with `-- --nocapture` to see this output.
        println!("Trade size: {} bytes", size);
        assert!(size > 0);
    }
}
