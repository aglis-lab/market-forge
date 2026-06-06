use std::vec;

use crate::core::{
    error, order,
    order_book::{OrderBook, common::ReplaceOrder, trade},
};

impl<T: order::Order> OrderBook<T> {
    pub fn replace_order(
        &mut self,
        replace_order: ReplaceOrder,
    ) -> Result<&vec::Vec<trade::Trade>, error::OrderError> {
        // Get order from order allocator, return early if order not found
        let order_quantity = match self.order_allocator.get_by_order_id(replace_order.order_id) {
            Some((_, order_node)) => order_node.order().quantity(),
            None => return Err(error::OrderError::OrderNotFound),
        };

        // Check if the new order is valid, return early if the new order is invalid
        let quantity_delta = replace_order.quantity_delta;
        let new_quantity = {
            if quantity_delta.is_positive() {
                order_quantity + quantity_delta as u64
            } else {
                let quantity_delta_abs = (-quantity_delta) as u64;
                if quantity_delta_abs >= order_quantity {
                    return Err(error::OrderError::InvalidQuantityDelta);
                }

                order_quantity - quantity_delta_abs
            }
        };

        // Cancel last order and get the cancel order, if cancel order not found, return early
        let mut cancel_order = self.cancel_order(replace_order.order_id)?;

        // Change quantity and price of the cancel order, return early if the new order is invalid
        cancel_order.set_quantity(new_quantity);
        if replace_order.new_price > 0 {
            cancel_order.set_price(replace_order.new_price);
        }

        // Insert the new order to book
        Ok(self.insert_order(&cancel_order))
    }
}
