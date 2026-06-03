use crate::{
    core::{order, order_allocator, order_book::OrderBook, order_match},
    utils,
};

impl<T: order::Order> OrderBook<T> {
    #[inline]
    pub fn insert_order(&mut self, order: &T) -> Option<Vec<order_match::OrderMatch>> {
        // Using slab allocator for performance
        let order_idx = self.order_allocator.insert(order.clone());

        // Check if matched
        let order_matches = self.process_order(order_idx, order);

        // Update Book Order
        if order.is_ephemeral_order() {
            self.order_allocator.try_remove(order_idx);
        } else if !self.update_book_order(order_idx) {
            self.order_allocator.try_remove(order_idx);
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
    fn update_book_order(&mut self, order_idx: order_allocator::AllocatorIndex) -> bool {
        let order = self.order_allocator.get(order_idx).unwrap();
        if order.quantity() == 0 {
            return false;
        }

        // Add Order
        if order.is_buy() {
            let key = &utils::ReverseOrd::new(order.price());
            self.bids
                .add_order(key, order_idx, order.id(), order.quantity());
        } else {
            let key = &order.price();
            self.asks
                .add_order(key, order_idx, order.id(), order.quantity());
        }

        return true;
    }

    #[inline]
    fn match_order(
        &mut self,
        allocator_idx: order_allocator::AllocatorIndex,
        top_price: order::Price,
        order_matches: &mut Vec<order_match::OrderMatch>,
    ) -> Option<()> {
        let (order_side, order_type, order_price, mut order_quantity) = self
            .order_allocator
            .get(allocator_idx)
            .map(|o| (o.order_side(), o.order_type(), o.price(), o.quantity()))
            .unwrap();
        if order_type.is_limit() {
            let is_match_price = self.is_match_price(&order_side, order_price, top_price);

            // Check if order is match price
            if !is_match_price {
                return None;
            }
        }

        // Get orders for the top price
        let orders = {
            if order_side.is_buy() {
                self.asks.get_orders_mut(&top_price).unwrap()
            } else {
                self.bids
                    .get_orders_mut(&utils::ReverseOrd::new(top_price))
                    .unwrap()
            }
        };

        // Set Order and total quantity
        let min_total_quantity = std::cmp::min(orders.orders_quantity(), order_quantity);
        orders.set_orders_quantity(orders.orders_quantity() - min_total_quantity);

        while orders.len() > 0 {
            let front_order_meta = orders.peek_front().unwrap();

            assert!(
                order_quantity > 0,
                "Order quantity should be greater than 0"
            );
            assert!(
                self.order_allocator
                    .contains(front_order_meta.allocator_idx()),
                "Order allocator should contain the front index"
            );

            let (front_order, order) = self
                .order_allocator
                .get2_mut(front_order_meta.allocator_idx(), allocator_idx)
                .unwrap();

            // Match the order with the front order
            let min_quantity = std::cmp::min(front_order.quantity(), order_quantity);
            front_order.set_quantity(front_order.quantity() - min_quantity);
            order_quantity -= min_quantity;

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
                let order_meta = orders.pop_front().unwrap();

                // Remove the order from the allocator
                self.order_allocator.try_remove(order_meta.allocator_idx());
            }

            // If the result order is fully matched, return None
            if order_quantity == 0 {
                break;
            }
        }

        // Remove the order from the book if it has no remaining quantity
        if orders.orders_quantity() == 0 {
            self.remove_orders(order_side.is_sell(), &top_price);
        }

        self.decrease_total_quantity(order_side.is_sell(), min_total_quantity);
        self.order_allocator
            .get_mut(allocator_idx)
            .unwrap()
            .set_quantity(order_quantity);
        if order_quantity == 0 {
            return None;
        }

        Some(())
    }
}
