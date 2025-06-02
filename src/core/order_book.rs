use std::cmp;

use slab::Slab;
use tabled::{builder::Builder, settings::Style};

use crate::{
    core::order::{self, Order, OrderId, OrderSide, Price, Quantity},
    core::order_error::OrderError,
    core::order_map::OrderMap,
    core::order_match::OrderMatch,
    core::orders::{Orders, SlabIndex},
    utils::ReverseOrd,
};

pub struct OrderBook<T: Order> {
    // Memory Allocator
    order_allocator: slab::Slab<T>,

    // Bids and Asks
    bids: OrderMap<ReverseOrd<Price>>,
    asks: OrderMap<Price>,

    // Stop Order
    stop_bids: OrderMap<ReverseOrd<Price>>,
    stop_asks: OrderMap<Price>,

    // Price
    current_market_price: Price,
}

// Public Function
impl<T: Order> OrderBook<T> {
    #[inline(always)]
    pub fn new(expected_peak_order: usize) -> Self {
        return OrderBook {
            order_allocator: slab::Slab::with_capacity(expected_peak_order),
            asks: OrderMap::new(),
            bids: OrderMap::new(),
            stop_asks: OrderMap::new(),
            stop_bids: OrderMap::new(),
            current_market_price: 0,
        };
    }

    #[inline(always)]
    pub fn current_market_price(&self) -> Price {
        return self.current_market_price;
    }

    pub fn set_market_price(&mut self, current_market_price: Price) {
        self.current_market_price = current_market_price
    }

    #[inline(always)]
    pub fn asks(&self) -> &OrderMap<Price> {
        return &self.asks;
    }

    #[inline(always)]
    pub fn bids(&self) -> &OrderMap<ReverseOrd<Price>> {
        return &self.bids;
    }

    #[inline(always)]
    pub fn stop_asks(&self) -> &OrderMap<Price> {
        return &self.stop_asks;
    }

    #[inline(always)]
    pub fn stop_bids(&self) -> &OrderMap<ReverseOrd<Price>> {
        return &self.stop_bids;
    }

    #[inline(always)]
    pub fn order_allocator(&self) -> &Slab<T> {
        return &self.order_allocator;
    }

    pub fn insert_order(&mut self, order: &T) -> Option<Vec<OrderMatch>> {
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

    pub fn replace_order(
        &mut self,
        order: &T,
        quantity_delta: i64,
        new_price: Price,
    ) -> Result<Vec<OrderMatch>, OrderError> {
        // Only quantity is changing
        let orders = self.get_orders(order).ok_or(OrderError::OrdersNotFound)?;

        // Get Order Meta
        // If already found, we should expect it's also exist at slab allocator
        let (order_index, order_meta) = orders
            .items()
            .iter()
            .enumerate()
            .find(|(_, item)| item.order_id() == order.id())
            .ok_or(OrderError::OrderNotFound)?;

        // Get Order inside Slab Allocator
        let slab_order = self
            .order_allocator
            .get(order_meta.slab_idx() as usize)
            .ok_or(OrderError::SlabOrderNotFound)?;

        // Check if quantity is still valid
        // New Order Quanttity
        let new_quantity = {
            if quantity_delta.is_positive() {
                slab_order
                    .quantity()
                    .saturating_add(quantity_delta as Quantity)
            } else {
                slab_order
                    .quantity()
                    .saturating_sub(quantity_delta as Quantity)
            }
        };

        if new_quantity == 0 {
            return Err(OrderError::OrderAlreadyFilled);
        }

        // New Order Quantity
        let new_orders_quantity = orders.orders_quantity() - slab_order.quantity();

        // New Total Quantity
        let new_total_quantity = self.total_quantity(order.is_buy()) - slab_order.quantity();

        // Create New Order
        let mut new_order = order.clone().with_quantity(new_quantity);
        if new_price != 0 {
            new_order = new_order.with_price(new_price);
        }

        // Delete last order
        // delete from slab allocator
        self.order_allocator.remove(order_meta.slab_idx() as usize);

        // delete from btreemap
        let orders = self.get_orders_mut(order).unwrap();
        orders.items_mut().remove(order_index);

        // Change Orders Quantity
        orders.set_orders_quantity(new_orders_quantity);

        // Change Total Quantity
        self.decrease_total_quantity(order.is_buy(), new_total_quantity);

        // Insert as new order
        let matches = self.insert_order(&new_order).unwrap_or_default();
        Ok(matches)
    }

    pub fn cancel_order(&mut self, order: &T) -> Result<T, OrderError> {
        // Get immutable orders
        let orders = self.get_orders(order).ok_or(OrderError::OrdersNotFound)?;

        // Get Order Index and Order Meta from orders
        let (order_index, order_meta) = orders
            .items()
            .iter()
            .enumerate()
            .find(|(_, item)| item.order_id() == order.id())
            .ok_or(OrderError::OrderNotFound)?;

        // Get Slab Order
        let slab_order = self
            .order_allocator
            .try_remove(order_meta.slab_idx() as usize)
            .ok_or(OrderError::SlabFailedRemoveOrder)?;

        // Get mutable orders
        let orders = self.get_orders_mut(&slab_order).unwrap();

        // Remove Order at orders
        orders.items_mut().remove(order_index);

        // Set Orders Quantity
        orders.set_orders_quantity(orders.orders_quantity() - slab_order.quantity());

        // Check if no order leave at orders
        if orders.len() == 0 {
            self.remove_orders(slab_order.is_buy(), &order.price());
        }

        // Decrease Total Quantity
        self.decrease_total_quantity(slab_order.is_buy(), slab_order.quantity());

        return Ok(slab_order);
    }

    /// Insert a stop order into the order book.
    /// Stop orders are stored in stop_bids or stop_asks depending on side.
    /// Returns true if the stop order was added.
    pub fn insert_stop_order(&mut self, order: &T) {
        let order_idx = self.order_allocator.insert(order.clone());

        // Add to stop order map
        if order.is_buy() {
            let key = &ReverseOrd::new(order.price());
            self.stop_bids
                .add_order(key, order_idx as SlabIndex, order.id(), order.quantity());
        } else {
            let key = &order.price();
            self.stop_asks
                .add_order(key, order_idx as SlabIndex, order.id(), order.quantity());
        }
    }

    pub fn recover_order_price(&self, order_side: OrderSide, order_id: OrderId) -> Option<Price> {
        if order_side.is_sell() {
            return self.asks.orders().iter().find_map(|(price, orders)| {
                for order in orders.items() {
                    if order.order_id() == order_id {
                        return Some(*price);
                    }
                }

                None
            });
        } else {
            return self.bids.orders().iter().find_map(|(price, orders)| {
                for order in orders.items() {
                    if order.order_id() == order_id {
                        return Some(price.0);
                    }
                }

                None
            });
        }
    }

    /// Trigger stop orders if the market price crosses their stop price.
    /// This should be called after each trade or price update.
    // pub fn trigger_stop_orders(&mut self) -> Vec<OrderMatch> {
    //     let mut triggered_matches = Vec::new();

    //     // Collect Bids Price
    //     let bids_prices = self
    //         .bids
    //         .collect_until_key(|key| self.current_market_price < key.0)
    //         .iter()
    //         .map(|item| item.0);

    //     triggered_matches
    // }

    // Optional: Validate cache consistency
    #[inline(always)]
    pub fn validate_cache(&self) -> Result<(), String> {
        self.asks.validate_cache()?;
        self.bids.validate_cache()?;

        return Ok(());
    }
}

// Implementation of the `OrderBook` struct, for managing bids and asks
impl<T: Order> OrderBook<T> {
    fn process_order(&mut self, order_idx: usize, order: &T) -> Vec<OrderMatch> {
        let mut order_matches: Vec<OrderMatch> = Vec::new();

        // Check if FOK or market
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
            let front_order_meta = orders.items().front().unwrap();

            assert!(
                order_quantity > 0,
                "Order quantity should be greater than 0"
            );
            assert!(
                self.order_allocator
                    .contains(front_order_meta.slab_idx() as usize),
                "Order allocator should contain the front index"
            );

            let (front_order, order) = self
                .order_allocator
                .get2_mut(front_order_meta.slab_idx() as usize, order_idx)
                .unwrap();

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
                let order_meta = orders.pop_front().unwrap();

                // Remove the order from the allocator
                self.order_allocator.remove(order_meta.slab_idx() as usize);
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
            .get_mut(order_idx)
            .unwrap()
            .set_quantity(order_quantity);
        if order_quantity == 0 {
            return None;
        }

        Some(())
    }

    #[inline(always)]
    fn get_orders(&self, order: &T) -> Option<&Orders> {
        if order.is_buy() {
            return self.bids.get_orders(&ReverseOrd::new(order.price()));
        } else {
            return self.asks.get_orders(&order.price());
        }
    }

    #[inline(always)]
    fn get_orders_mut(&mut self, order: &T) -> Option<&mut Orders> {
        if order.is_buy() {
            return self.bids.get_orders_mut(&ReverseOrd::new(order.price()));
        } else {
            return self.asks.get_orders_mut(&order.price());
        }
    }

    #[inline(always)]
    pub fn peek_top_price(&self, is_bids: bool) -> Option<&Price> {
        if is_bids {
            return self.bids.peek_key().map(|i| &i.0);
        } else {
            return self.asks.peek_key();
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
    fn total_quantity(&self, is_bids: bool) -> Quantity {
        if is_bids {
            return self.bids.total_quantity();
        } else {
            return self.asks.total_quantity();
        }
    }

    #[inline(always)]
    fn set_total_quantity(&mut self, is_bids: bool, new_quantity: Quantity) {
        if is_bids {
            self.bids.set_total_quantity(new_quantity);
        } else {
            self.asks.set_total_quantity(new_quantity);
        }
    }

    #[inline(always)]
    fn decrease_total_quantity(&mut self, is_bids: bool, quantity: Quantity) {
        if is_bids {
            self.set_total_quantity(is_bids, self.bids.total_quantity() - quantity);
        } else {
            self.set_total_quantity(is_bids, self.asks.total_quantity() - quantity);
        }
    }

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

        // Add Order
        if order.is_buy() {
            let key = &ReverseOrd::new(order.price());
            self.bids
                .add_order(key, order_idx as SlabIndex, order.id(), order.quantity());
        } else {
            let key = &order.price();
            self.asks
                .add_order(key, order_idx as SlabIndex, order.id(), order.quantity());
        }

        return true;
    }

    #[inline(always)]
    fn remove_orders(&mut self, is_bids: bool, top_price: &Price) {
        if is_bids {
            self.bids.remove_orders(&ReverseOrd::new(*top_price));
        } else {
            self.asks.remove_orders(&top_price);
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
