use crate::core::{error, order, order_book::OrderBook};

impl<T: order::Order> OrderBook<T> {
    pub fn cancel_order(&mut self, order_id: order::OrderId) -> Result<T, error::OrderError> {
        // Get order from order allocator, return early if order not found
        // Delete order from allocator
        let (order_allocator_idx, order_node) =
            match self.order_allocator.try_remove_by_order_id(order_id) {
                Some((idx, order_node)) => (idx, order_node),
                None => return Err(error::OrderError::OrderNotFound),
            };
        let order = order_node.order();

        // Get prev order and next order from order allocator and change prev and next order link
        if let Some(prev_order_node) = order_node
            .prev_idx()
            .and_then(|idx| self.order_allocator.get_mut(idx))
        {
            prev_order_node.set_next_idx(order_node.next_idx());
        }

        if let Some(next_order_node) = order_node
            .next_idx()
            .and_then(|idx| self.order_allocator.get_mut(idx))
        {
            next_order_node.set_prev_idx(order_node.prev_idx());
        }

        // Get price level and decrease quantity, remove price level if no order left
        // bids is buy side and asks is sell side, so we check order side to get price level
        let price_level = match self.get_price_level_mut(order.order_side().is_buy(), order.price())
        {
            Some(price_level) => price_level,
            None => return Err(error::OrderError::PriceLevelNotFound),
        };

        price_level.set_len(price_level.len() - 1);
        if price_level.is_empty() {
            self.remove_price_level(order.is_buy(), &order.price());
        } else {
            price_level.set_quantity(price_level.quantity() - order.quantity());
            if price_level.head() == order_allocator_idx {
                price_level.set_head(
                    order_node
                        .next_idx()
                        .ok_or(error::OrderError::OrderNextIdxNotFound)?,
                );
            } else if price_level.tail() == order_allocator_idx {
                price_level.set_tail(
                    order_node
                        .prev_idx()
                        .ok_or(error::OrderError::OrderPrevIdxNotFound)?,
                );
            }
        }

        // Decrease total quantity
        self.decrease_total_quantity(order.is_buy(), order.quantity());

        // Return the cancel order
        Ok(order.clone())
    }
}
