use crate::{
    core::{order, order_allocator, order_book::OrderBook, order_match},
    utils,
};

struct ProcessMatchResult {
    next_idx: Option<order_allocator::AllocatorIndex>,
    head_order_remaining: order::Quantity,
    matched_quantity: order::Quantity,
}

impl ProcessMatchResult {
    #[inline(always)]
    fn is_head_fully_matched(&self) -> bool {
        self.head_order_remaining == 0
    }
}

impl<T: order::Order> OrderBook<T> {
    pub fn insert_order(&mut self, order: &T) -> Vec<order_match::OrderMatch> {
        let mut matches = Vec::new();
        // Check if FOK
        // return early if not match quantity
        if order.is_fill_or_kill() && !self.has_sufficient_quantity(order) {
            return matches;
        }

        // Match Order
        let mut order = order.clone();
        while let Some(&top_price) = self.peek_top_price(order.is_sell()) {
            if order.is_limit_price() {
                if !self.is_match_price(&order.order_side(), order.price(), top_price) {
                    break;
                }
            }

            if !self.process_match_price(top_price, &mut order, &mut matches) {
                break;
            }
        }

        // Partial match only for non-IOC or FOK order
        // Insert the remaining order to book if it is not fully matched
        if !order.is_immediate_or_cancel() && order.quantity() > 0 {
            let allocator_idx = self
                .order_allocator
                .insert(order_allocator::OrderNode::new(order.clone()));

            // Update Price Level
            let prev_tail_idx = self.upsert_price_level(allocator_idx, &order);

            // Link Order Node
            if let Some(prev_tail_idx) = prev_tail_idx {
                self.order_allocator
                    .get_mut(prev_tail_idx)
                    .unwrap()
                    .set_next_idx(Some(allocator_idx));

                self.order_allocator
                    .get_mut(allocator_idx)
                    .unwrap()
                    .set_prev_idx(Some(prev_tail_idx));
            }
        }

        return matches;
    }

    // Return true if has sufficient quantity to match, false otherwise
    fn process_match_price(
        &mut self,
        top_price: order::Price,
        incoming_order: &mut T,
        matches: &mut Vec<order_match::OrderMatch>,
    ) -> bool {
        log::debug!(
            "Processing match price: {}, incoming order: {:?}",
            top_price,
            incoming_order
        );

        let mut price_level = self
            .get_price_level(incoming_order.order_side().is_sell(), top_price)
            .unwrap()
            .clone();

        // Check if price level and order have sufficient quantity to match
        while price_level.quantity() > 0 && incoming_order.quantity() > 0 {
            log::debug!(
                "Matching at price level: {}, price level quantity: {}, incoming order quantity: {}",
                top_price,
                price_level.quantity(),
                incoming_order.quantity()
            );

            assert!(
                self.order_allocator.contains(price_level.head()),
                "Order allocator should contain the front index, alloc_idx: {}",
                price_level.head(),
            );

            let match_result = {
                // Get head order and match with incoming order
                let head_order_node = self
                    .order_allocator
                    .get_mut(price_level.head())
                    .expect("Order allocator should contain the head index");

                let min_quantity = std::cmp::min(
                    head_order_node.order().quantity(),
                    incoming_order.quantity(),
                );

                // Check all remaining quantity and next index
                let head_order_remaining = head_order_node.order().quantity() - min_quantity;
                let next_idx = head_order_node.next_idx();

                // Update head order quantity and incoming order quantity
                price_level.set_quantity(price_level.quantity() - min_quantity);
                incoming_order.set_quantity(incoming_order.quantity() - min_quantity);
                head_order_node
                    .order_mut()
                    .set_quantity(head_order_remaining);

                // Add the match result to the order matches
                matches.push(order_match::OrderMatch {
                    order_side: incoming_order.order_side(),
                    price: top_price,
                    quantity: min_quantity,
                    match_from_id: incoming_order.id(),
                    match_to_id: head_order_node.order().id(),
                });

                ProcessMatchResult {
                    next_idx,
                    head_order_remaining,
                    matched_quantity: min_quantity,
                }
            };

            log::debug!(
                "Match result: head_order_remaining: {}, next_idx: {:?} with price level: {:?}",
                match_result.head_order_remaining,
                match_result.next_idx,
                price_level,
            );

            // Remove the head order from book if it is fully matched
            if match_result.is_head_fully_matched() {
                self.order_allocator
                    .try_remove(price_level.head())
                    .expect("order allocator should contain the head index");

                price_level.set_len(price_level.len() - 1);

                if let Some(next_idx) = match_result.next_idx {
                    if let Some(next_node) = self.order_allocator.get_mut(next_idx) {
                        next_node.set_prev_idx(None);
                    }

                    price_level.set_head(next_idx);
                }
            }

            // Decrease the bookside total quantity by the matched quantity
            self.decrease_total_quantity(incoming_order.is_sell(), match_result.matched_quantity);
        }

        if price_level.is_empty() {
            self.remove_price_level(incoming_order.is_sell(), &top_price);
        } else if let Some(new_price_level) =
            self.get_price_level_mut(incoming_order.is_sell(), top_price)
        {
            new_price_level.set_head(price_level.head());
            new_price_level.set_len(price_level.len());
            new_price_level.set_quantity(price_level.quantity());
        }

        return price_level.is_empty() && incoming_order.quantity() > 0;
    }

    #[inline(always)]
    fn upsert_price_level(
        &mut self,
        allocator_idx: order_allocator::AllocatorIndex,
        order: &T,
    ) -> Option<order_allocator::AllocatorIndex> {
        let price = order.price();
        if order.is_buy() {
            return self.bids.upsert_price_level(
                utils::ReverseOrd::new(price),
                allocator_idx,
                order.quantity(),
            );
        } else {
            return self
                .asks
                .upsert_price_level(price, allocator_idx, order.quantity());
        }
    }
}
