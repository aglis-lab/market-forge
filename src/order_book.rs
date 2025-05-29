use std::cmp;

use slab::Slab;
use tabled::{builder::Builder, settings::Style};

use crate::{
    order::{self, Order, Price, Quantity},
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

// Implementation of the `OrderBook` struct, for managing bids and asks
impl<T: Order> OrderBook<T> {
    #[inline(always)]
    pub fn new(expected_peak_order: usize) -> Self {
        return OrderBook {
            order_allocator: slab::Slab::with_capacity(expected_peak_order),
            asks: OrderMap::new(),
            bids: OrderMap::new(),
        };
    }

    #[inline(always)]
    pub fn asks(&self) -> &OrderMap<Price> {
        return &self.asks;
    }

    #[inline(always)]
    pub fn bids(&self) -> &OrderMap<ReverseOrd<Price>> {
        return &self.bids;
    }

    pub fn order_allocator(&self) -> &Slab<T> {
        &self.order_allocator
    }

    pub fn add(&mut self, order: &T) -> Option<Vec<OrderMatch>> {
        // Using slab allocator for performance
        let order_idx = self.order_allocator.insert(order.clone());

        // Check if matched
        let order_matches = self.process_order(order_idx, order);

        // Update Book Order
        if order.is_ephemeral_order() {
            self.order_allocator.remove(order_idx);
        } else if !self.update_book_order(order_idx) {
            self.order_allocator.remove(order_idx);
        }

        // Do something with order match
        if order_matches.is_empty() {
            return None;
        }

        return Some(order_matches);
    }

    fn process_order(&mut self, order_idx: usize, order: &T) -> Vec<OrderMatch> {
        let mut order_matches: Vec<OrderMatch> = Vec::new();

        // Check if FOK or market
        // return early if not match quantity
        if order.is_fill_or_kill() && !self.has_sufficient_quantity(order) {
            return order_matches;
        }

        // Match Order
        loop {
            let top_price = self.peek_top_price(order.is_sell());
            if top_price == None {
                break;
            }

            if self.match_order(order_idx, *top_price.unwrap(), &mut order_matches) == None {
                break;
            }
        }

        // Return order match
        return order_matches;
    }

    fn match_order(
        &mut self,
        order_idx: usize,
        top_price: Price,
        order_matches: &mut Vec<OrderMatch>,
    ) -> Option<()> {
        let (order_side, order_type, order_price, mut order_quantity) = self
            .order_allocator
            .get(order_idx)
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
                    .get_orders_mut(&ReverseOrd::new(top_price))
                    .unwrap()
            }
        };

        // Set Order and total quantity
        let min_total_quantity = cmp::min(orders.orders_quantity(), order_quantity);
        orders.set_orders_quantity(orders.orders_quantity() - min_total_quantity);

        while orders.len() > 0 {
            let front_idx = orders.items().front().unwrap().clone();

            assert!(
                order_quantity > 0,
                "Order quantity should be greater than 0"
            );
            assert!(
                self.order_allocator.contains(front_idx),
                "Order allocator should contain the front index"
            );

            let (front_order, order) = self.order_allocator.get2_mut(front_idx, order_idx).unwrap();

            // Match the order with the front order
            let min_quantity = cmp::min(front_order.quantity(), order_quantity);
            front_order.set_quantity(front_order.quantity() - min_quantity);
            order_quantity -= min_quantity;

            // Add the match result to the order matches
            order_matches.push(OrderMatch {
                order_side: order.order_side(),
                price: top_price,
                quantity: min_quantity,
                match_from_id: order.id(),
                match_to_id: front_order.id(),
            });

            // If the front order is fully matched, remove it from the queue
            if front_order.quantity() == 0 {
                let order_idx = orders.pop_front();
                // Remove the order from the allocator
                self.order_allocator.remove(order_idx.unwrap());
            }

            // If the result order is fully matched, return None
            if order_quantity == 0 {
                break;
            }
        }

        // Remove the order from the book if it has no remaining quantity
        if orders.orders_quantity() == 0 {
            self.remove_order(order_side.is_sell(), &top_price);
        }

        self.decrease_total_quantity(order_side.is_sell(), min_total_quantity);
        self.order_allocator
            .get_mut(order_idx)
            .unwrap()
            .set_quantity(order_quantity);
        if order_quantity == 0 {
            return None;
        }

        Some(())
    }

    #[inline(always)]
    pub fn peek_top_price(&self, is_bids: bool) -> Option<&Price> {
        if is_bids {
            return self.bids.peek().map(|i| &i.0);
        } else {
            return self.asks.peek();
        }
    }

    #[inline(always)]
    pub fn has_sufficient_quantity(&self, order: &T) -> bool {
        let quantity: Quantity = {
            if order.is_limit_price() {
                if order.is_buy() {
                    self.asks.collect_quantity_match_price(
                        &order.price(),
                        &order.order_side(),
                        &order.quantity(),
                    )
                } else {
                    self.bids.collect_quantity_match_price(
                        &ReverseOrd::new(order.price()),
                        &order.order_side(),
                        &order.quantity(),
                    )
                }
            } else {
                // Market Order
                if order.is_buy() {
                    self.asks.total_quantity()
                } else {
                    self.bids.total_quantity()
                }
            }
        };

        return quantity >= order.quantity();
    }

    #[inline(always)]
    fn decrease_total_quantity(&mut self, is_bids: bool, quantity: Quantity) {
        if is_bids {
            self.bids
                .set_total_quantity(self.bids.total_quantity() - quantity);
        } else {
            self.asks
                .set_total_quantity(self.asks.total_quantity() - quantity);
        }
    }

    // #[inline(always)]
    // fn get_order_mut(&mut self, is_bids: bool, key: &Price) -> &mut Orders {
    //     if is_bids {
    //         return self.bids.get_orders_mut(&ReverseOrd::new(*key)).unwrap();
    //     } else {
    //         return self.asks.get_orders_mut(key).unwrap();
    //     }
    // }

    #[inline(always)]
    fn is_match_price(
        &self,
        order_side: &order::OrderSide,
        order_price: Price,
        top_price: Price,
    ) -> bool {
        if order_side.is_buy() && order_price >= top_price {
            return true;
        }

        if order_side.is_sell() && order_price <= top_price {
            return true;
        }

        false
    }

    #[inline(always)]
    fn update_book_order(&mut self, order_idx: usize) -> bool {
        let order = self.order_allocator.get(order_idx).unwrap();
        if order.quantity() == 0 {
            return false;
        }

        if order.is_buy() {
            let key = &ReverseOrd::new(order.price());
            self.bids.add_order(key, order_idx, order.quantity());
        } else {
            let key = &order.price();
            self.asks.add_order(key, order_idx, order.quantity());
        }

        return true;
    }

    #[inline(always)]
    fn remove_order(&mut self, is_bids: bool, top_price: &Price) {
        if is_bids {
            self.bids.remove_order(&ReverseOrd::new(*top_price));
        } else {
            self.asks.remove_order(&top_price);
        }
    }
}

impl<T: Order> std::fmt::Display for OrderBook<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = writeln!(
            f,
            "Bids: {}, Bids Qty: {}, Asks: {}, Asks: Qty: {}, Alloc: {}",
            self.bids().len(),
            self.bids().total_quantity(),
            self.asks().len(),
            self.asks().total_quantity(),
            self.order_allocator().len()
        );

        let mut builder = Builder::new();
        builder.push_record(["Bids", "Total", "Asks", "Total"]);

        // Reverse bids for descending order (as bid books are usually displayed)
        let bids: Vec<_> = self.bids().orders().iter().collect();
        let asks: Vec<_> = self.asks().orders().iter().collect();
        let max_len = bids.len().max(asks.len());

        for i in 0..max_len {
            let (bid_price, bid_qty) = bids
                .get(i)
                .map(|(p, o)| {
                    (
                        p.0.to_string(),
                        format!("{}({})", o.orders_quantity().to_string(), o.len()),
                    )
                })
                .unwrap_or(("".to_string(), "".to_string()));

            let (ask_price, ask_qty) = asks
                .get(i)
                .map(|(p, o)| {
                    (
                        p.to_string(),
                        format!("{}({})", o.orders_quantity().to_string(), o.len()),
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
