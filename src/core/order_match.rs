use crate::core::order::{OrderId, OrderSide, Price, Quantity};

#[derive(Debug, PartialEq)]
pub struct OrderMatch {
    pub order_side: OrderSide,
    pub price: Price,
    pub quantity: Quantity,

    pub match_from_id: OrderId,
    pub match_to_id: OrderId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn show_order_match_size() {
        let size = size_of::<OrderMatch>();
        // Print size; run tests with `-- --nocapture` to see this output.
        println!("OrderMatch size: {} bytes", size);
        assert!(size > 0);
    }
}
