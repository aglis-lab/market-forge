use crate::{
    core::{order, order_allocator, order_book::OrderBook, order_match},
    utils,
};

impl<T: order::Order> OrderBook<T> {
    #[inline]
    pub fn insert_order(&mut self, order: &T) -> Option<Vec<order_match::OrderMatch>> {
        // Using slab allocator for performance
        let allocator_idx = self
            .order_allocator
            .insert(order_allocator::OrderNode::new(order.clone()));

        // Check if matched
        let order_matches = self.process_order(allocator_idx, order);

        // Update Book Order
        if order.is_ephemeral_order() {
            self.order_allocator.try_remove(allocator_idx);
        } else if !self.update_book_order(allocator_idx) {
            self.order_allocator.try_remove(allocator_idx);
        }

        // Do something with order match
        if order_matches.is_empty() {
            return None;
        }

        return Some(order_matches);
    }

    #[inline]
    fn process_order(
        &mut self,
        allocator_idx: order_allocator::AllocatorIndex,
        order: &T,
    ) -> Vec<order_match::OrderMatch> {
        let mut order_matches: Vec<order_match::OrderMatch> = Vec::new();

        // TODO: Check if FOK or market
        // return early if not match quantity
        // let order = self.order_allocator.get(order_idx).unwrap();
        if order.is_fill_or_kill() && !self.has_sufficient_quantity(order) {
            return order_matches;
        }

        // Match Order
        loop {
            let top_price = self.peek_top_price(order.is_sell());
            if top_price == None {
                break;
            }

            if self.match_order(allocator_idx, *top_price.unwrap(), &mut order_matches) == None {
                break;
            }
        }

        // Return order match
        return order_matches;
    }

    #[inline(always)]
    fn update_book_order(&mut self, allocator_idx: order_allocator::AllocatorIndex) -> bool {
        let order = self.order_allocator.get(allocator_idx).unwrap().order();
        if order.quantity() == 0 {
            return false;
        }

        // Upsert price level and update order node prev and update prev order node next to current order node allocator idx
        if order.is_buy() {
            let prev_tail_idx = self.bids.upsert_price_level(
                utils::ReverseOrd::new(order.price()),
                allocator_idx,
                order.quantity(),
            );
            if let Some(prev_tail_idx) = prev_tail_idx {
                self.order_allocator
                    .get_mut(prev_tail_idx)
                    .unwrap()
                    .set_next_idx(Some(allocator_idx));
            }
        } else {
            let prev_tail_idx =
                self.asks
                    .upsert_price_level(order.price(), allocator_idx, order.quantity());
            if let Some(prev_tail_idx) = prev_tail_idx {
                self.order_allocator
                    .get_mut(prev_tail_idx)
                    .unwrap()
                    .set_next_idx(Some(allocator_idx));
            }
        }

        return true;
    }

    #[inline]
    fn match_order(
        &mut self,
        incoming_allocator_idx: order_allocator::AllocatorIndex,
        top_price: order::Price,
        order_matches: &mut Vec<order_match::OrderMatch>,
    ) -> Option<()> {
        let (
            incoming_order_side,
            incoming_order_type,
            incoming_order_price,
            mut incoming_order_quantity,
        ) = self
            .order_allocator
            .get(incoming_allocator_idx)
            .map(|o| {
                let order = o.order();

                (
                    order.order_side(),
                    order.order_type(),
                    order.price(),
                    order.quantity(),
                )
            })
            .unwrap();
        if incoming_order_type.is_limit() {
            let is_match_price =
                self.is_match_price(&incoming_order_side, incoming_order_price, top_price);

            // Check if order is match price
            if !is_match_price {
                return None;
            }
        }

        // Get orders for the top price
        let price_level = {
            if incoming_order_side.is_buy() {
                self.asks.get_price_level_mut(&top_price).unwrap()
            } else {
                self.bids
                    .get_price_level_mut(&utils::ReverseOrd::new(top_price))
                    .unwrap()
            }
        };

        // Set Order and total quantity
        let min_total_quantity = std::cmp::min(price_level.quantity(), incoming_order_quantity);
        price_level.set_quantity(price_level.quantity() - min_total_quantity);

        while price_level.len() > 0 {
            let front_level_alloc_idx = price_level.head();

            assert!(
                incoming_order_quantity > 0,
                "Order quantity should be greater than 0"
            );
            assert!(
                self.order_allocator.contains(front_level_alloc_idx),
                "Order allocator should contain the front index, alloc_idx: {}\n{}",
                front_level_alloc_idx,
                self.order_allocator,
            );

            let (front_node_order, incoming_node_order) = self
                .order_allocator
                .get2_mut(front_level_alloc_idx, incoming_allocator_idx)
                .unwrap();
            let front_order = front_node_order.order_mut();
            let order = incoming_node_order.order_mut();

            // Match the order with the front order
            let min_quantity = std::cmp::min(front_order.quantity(), incoming_order_quantity);
            front_order.set_quantity(front_order.quantity() - min_quantity);
            incoming_order_quantity -= min_quantity;

            // Add the match result to the order matches
            order_matches.push(order_match::OrderMatch {
                order_side: order.order_side(),
                price: top_price,
                quantity: min_quantity,
                match_from_id: order.id(),
                match_to_id: front_order.id(),
            });

            // If the front order is fully matched, remove it from the queue
            if front_order.quantity() == 0 {
                let next_alloc_idx = front_node_order.next_idx().unwrap_or_else(|| 0);

                price_level.set_head(next_alloc_idx);

                // Remove the order from the allocator
                self.order_allocator.try_remove(front_level_alloc_idx);
            }

            // If the result order is fully matched, break the loop
            if incoming_order_quantity == 0 {
                break;
            }
        }

        // Remove the order from the book if it has no remaining quantity
        if price_level.quantity() == 0 {
            self.remove_price(incoming_order_side.is_sell(), &top_price);
        }

        self.decrease_total_quantity(incoming_order_side.is_sell(), min_total_quantity);
        self.order_allocator
            .get_mut(incoming_allocator_idx)
            .unwrap()
            .order_mut()
            .set_quantity(incoming_order_quantity);
        if incoming_order_quantity == 0 {
            return None;
        }

        Some(())
    }
}
