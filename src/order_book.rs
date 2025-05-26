use std::{
    cmp,
    collections::{BTreeMap, BinaryHeap},
};

use tabled::{builder::Builder, settings::Style};

use crate::{
    order::{Order, Price},
    order_map::OrderMap,
    order_match::OrderMatch,
    orders::Orders,
    utils::ReverseOrd,
};

pub struct OrderBook<T: Order> {
    order_allocator: slab::Slab<T>,
    bids: OrderMap<ReverseOrd<Price>>,
    asks: OrderMap<Price>,
}

// // Implementation of the `OrderBook` struct, for managing order allocation
impl<T: Order> OrderBook<T> {
    //     // Allocate a new order in the slab allocator
    //     fn allocte_order(&mut self, new_order: &T) -> usize {
    //         return self.order_allocator.insert(new_order.clone());
    //     }

    // Get an order by its ID from the slab allocator
    fn get_mut_order(&mut self, id: usize) -> Option<&mut T> {
        return self.order_allocator.get_mut(id);
    }

    // Get an order by its ID from the slab allocator
    fn get_order(&self, id: usize) -> Option<&T> {
        return self.order_allocator.get(id);
    }

    //     fn remove_order(&mut self, order_idx: usize) -> T {
    //         return self.order_allocator.remove(order_idx);
    //     }
}

// Implementation of the `OrderBook` struct, for managing bids and asks
impl<T: Order> OrderBook<T> {
    pub fn new(expected_peak_order: usize) -> Self {
        return OrderBook {
            order_allocator: slab::Slab::with_capacity(expected_peak_order),
            asks: OrderMap::new(),
            bids: OrderMap::new(),
        };
    }

    pub fn asks(&self) -> &OrderMap<Price> {
        return &self.asks;
    }

    pub fn bids(&self) -> &OrderMap<ReverseOrd<Price>> {
        return &self.bids;
    }

    pub fn add(&mut self, new_order: T) -> Option<Vec<OrderMatch>> {
        // Using slab allocator for performance
        let order_idx = self.order_allocator.insert(new_order);

        // Check if matched
        let order_matches = self.process_order(order_idx);

        // Update Book Order
        if !self.update_book_order(order_idx) {
            self.order_allocator.remove(order_idx);
        }

        // Do something with order match
        if order_matches.is_empty() {
            return None;
        }

        return Some(order_matches);
    }

    fn process_order(&mut self, order_idx: usize) -> Vec<OrderMatch> {
        let mut order_matches: Vec<OrderMatch> = Vec::new();

        // Check if order is buy or sell
        let result = self
            .order_allocator
            .get_mut(order_idx)
            .expect("Order not found");
        if result.is_buy() {
            loop {
                let top_price = {
                    let peek = self.asks.peek();
                    if peek.is_none() {
                        break;
                    }
                    *peek.unwrap()
                };

                if self.match_order(order_idx, top_price, &mut order_matches) == None {
                    break;
                }
            }
        } else {
            // for (key, orders) in self.bids.iter_mut().rev() {
            //     if Self::match_order(key, orders, result, &mut order_matches, &mut keys_to_remove)
            //         == None
            //     {
            //         break;
            //     }
            // }

            // for key in keys_to_remove {
            //     self.bids.remove_order(&key);
            // }
        }

        return order_matches;
    }

    fn match_order(
        &mut self,
        order_idx: usize,
        top_price: Price,
        order_matches: &mut Vec<OrderMatch>,
    ) -> Option<()> {
        let is_match_price = self
            .order_allocator
            .get(order_idx)
            .expect("Order not found")
            .is_match_price(&top_price);

        // Check if order is match price
        if !is_match_price {
            return None;
        }

        // Get orders for the top price
        let (order_side, quantity) = self
            .order_allocator
            .get(order_idx)
            .map(|o| (o.order_side(), o.quantity()))
            .unwrap();
        let orders: &mut Orders;
        if order_side.is_buy() {
            orders = self
                .asks
                .get_orders_mut(&top_price)
                .expect("msg: No orders found for the top price");
        } else {
            orders = self
                .bids
                .get_orders_mut(&top_price)
                .expect("No orders found for the top price");
        }

        // Set total quantity
        let min_quantity = cmp::min(orders.total_quantity(), quantity);
        orders.set_total_quantity(orders.total_quantity() - min_quantity);

        while orders.len() > 0 {
            let front = orders.items().front().expect("No orders in the queue");

            let (front_order, order) = self
                .order_allocator
                .get2_mut(front.clone(), order_idx)
                .unwrap();

            // Match the order with the front order
            let match_result = front_order.match_order(order);

            // If the front order is fully matched, remove it from the queue
            if front_order.quantity() == 0 {
                orders.pop_front();
            }

            // If the result order is fully matched, return None
            if order.quantity() == 0 {
                return None;
            }

            // Add the match result to the order matches
            order_matches.push(match_result);
        }

        // Simulate removal
        if orders.total_quantity() == 0 {
            if order_side.is_buy() {
                self.bids.remove_order(&ReverseOrd(top_price), &top_price);
            } else {
                self.asks.remove_order(&top_price, &top_price);
            }
        }

        let latest_order_quantity = self
            .order_allocator
            .get(order_idx)
            .map(|o| o.quantity())
            .unwrap();
        if latest_order_quantity == 0 {
            return None;
        }

        Some(())
    }

    fn update_book_order(&mut self, order_idx: usize) -> bool {
        let order = self.order_allocator.get(order_idx).unwrap();
        if order.quantity() == 0 {
            return false;
        }

        let last_orders: Option<&mut Orders>;
        if order.is_buy() {
            self.bids.add_order(
                &ReverseOrd::new(order.price()),
                order.price(),
                order_idx,
                order.quantity(),
            );
        } else {
            self.asks
                .add_order(&order.price(), order.price(), order_idx, order.quantity());
        }

        // if last_orders.is_none() {
        //     let mut orders = Orders::new();
        //     orders.add(order_idx, order.quantity());

        //     if order.is_buy() {
        //         self.bids.insert(order.price(), orders);
        //     } else {
        //         self.asks.insert(order.price(), orders);
        //     }
        // } else {
        //     last_orders.unwrap().add(order_idx, order.quantity());
        // }

        return true;
    }
}

impl<T: Order> std::fmt::Display for OrderBook<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = Builder::new();
        builder.push_record(["Bids", "Total", "Asks", "Total"]);

        // Reverse bids for descending order (as bid books are usually displayed)
        let mut bids = self.bids.order_prices().iter();
        let mut asks = self.asks.order_prices().iter();
        let max_len = bids.len().max(asks.len());

        for i in 0..max_len {
            let (bid_price, bid_qty) = bids
                .nth(i)
                .map(|p| {
                    let o = self.bids.get_orders(&p.value()).expect("Order not found");

                    (
                        p.value().to_string(),
                        format!("{}({})", o.total_quantity().to_string(), o.len()),
                    )
                })
                .unwrap_or(("".to_string(), "".to_string()));

            let (ask_price, ask_qty) = asks
                .nth(i)
                .map(|p| {
                    let o = self.asks.get_orders(&p).expect("Order not found");

                    (
                        p.to_string(),
                        format!("{}({})", o.total_quantity().to_string(), o.len()),
                    )
                })
                .unwrap_or(("".to_string(), "".to_string()));

            builder.push_record([bid_price, bid_qty, ask_price, ask_qty]);
        }

        let mut table = builder.build();
        let temp = table.with(Style::modern_rounded());
        write!(f, "{temp}")
    }
}
