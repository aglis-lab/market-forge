use crate::core::{order, order_book::OrderBook, order_error};

impl<T: order::Order> OrderBook<T> {
    pub fn cancel_order(&mut self, order: &T) -> Result<T, order_error::OrderError> {
        // Get Order Index and Order Meta from orders
        // TODO: Refactor to return order allocator index and order id directly from orders
        let (order_node_alloc_idx, prev_order_node_idx, next_order_node_idx) = {
            let (order_node_alloc_idx, order_node) = self
                .order_allocator
                .get_by_order_id(order.id())
                .ok_or(order_error::OrderError::OrderNotFound)?;
            (
                order_node_alloc_idx,
                order_node.prev_idx(),
                order_node.next_idx(),
            )
        };

        // Set Prev Order next variable to current order next variable
        if let Some(prev_order_node_idx) = prev_order_node_idx {
            self.order_allocator
                .get_mut(prev_order_node_idx)
                .ok_or(order_error::OrderError::OrderNotFound)?
                .set_next_idx(next_order_node_idx);
        }

        // Set Next Order prev variable to current order prev variable
        if let Some(next_order_node_idx) = next_order_node_idx {
            self.order_allocator
                .get_mut(next_order_node_idx)
                .ok_or(order_error::OrderError::OrderNotFound)?
                .set_prev_idx(prev_order_node_idx);
        }

        // Remove Order from Slab Allocator
        self.order_allocator.try_remove(order_node_alloc_idx);

        // Get mutable orders
        let price_level = self
            .get_price_levels_mut(order)
            .ok_or(order_error::OrderError::OrdersNotFound)?;

        price_level.set_quantity(price_level.quantity() - order.quantity());

        // Remove Slab Order
        // let slab_order = self
        //     .order_allocator
        //     .try_remove(order_meta.allocator_idx())
        //     .ok_or(order_error::OrderError::SlabFailedRemoveOrder)?;

        // Get mutable orders
        // let orders = self.get_orders_mut(&slab_order).unwrap();

        // Remove Order at orders
        // orders.remove_index(order_index);

        // Set Orders Quantity
        // orders.set_orders_quantity(orders.orders_quantity() - slab_order.quantity());

        // Check if no order leave at orders
        if price_level.len() == 0 {
            self.remove_price(order.is_buy(), &order.price());
        }

        // Decrease Total Quantity
        self.decrease_total_quantity(order.is_buy(), order.quantity());

        return Ok(order.clone());
    }
}
