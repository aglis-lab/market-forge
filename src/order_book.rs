use std::collections::BTreeMap;

use tabled::{builder::Builder, settings::Style};

use crate::{
    order::{Order, Price},
    order_match::OrderMatch,
    orders::Orders,
    utils::ReverseOrd,
};

type OrderMapBids<T> = BTreeMap<ReverseOrd<Price>, Orders<T>>;
type OrderMapAsks<T> = BTreeMap<Price, Orders<T>>;

pub struct OrderBook<T: Order> {
    bids: OrderMapBids<T>,
    asks: OrderMapAsks<T>,
}

impl<T: Order> OrderBook<T> {
    pub fn new() -> Self {
        return OrderBook {
            asks: OrderMapAsks::new(),
            bids: OrderMapBids::new(),
        };
    }

    pub fn asks(&self) -> &OrderMapAsks<T> {
        return &self.asks;
    }

    pub fn bids(&self) -> &OrderMapBids<T> {
        return &self.bids;
    }

    pub fn add(&mut self, new_order: &T) -> Option<Vec<OrderMatch>> {
        if new_order.quantity() == 0 {
            return None;
        }

        // Check if matched
        let mut result = new_order.clone();
        let order_matches = self.match_order(&mut result);

        // Add Order
        if result.quantity() > 0 {
            self.add_order(&result);
        }

        // Do something with order match
        if order_matches.is_empty() {
            return None;
        }

        return Some(order_matches);
    }

    fn match_order(&mut self, result: &mut T) -> Vec<OrderMatch> {
        if result.is_buy() {
            return self.match_order_side_buy(result);
        }

        return self.match_order_side_sell(result);
    }

    fn match_order_side_buy(&mut self, result: &mut T) -> Vec<OrderMatch> {
        let mut order_matches: Vec<OrderMatch> = Vec::new();
        // let mut keys_to_remove = Vec::new();

        for (key, orders) in self.asks.iter_mut() {
            let is_price_match = result.is_match_price(*key);

            // Check if price no match
            if !is_price_match {
                break;
            }

            if is_price_match && orders.total_quantity() > 0 {
                let val = orders.match_order(result);

                // Extend Order Matches
                order_matches.extend(val);

                // // Check if total quantity is 0
                // if orders.total_quantity() == 0 {
                //     keys_to_remove.push(*key);
                // }
            }

            if result.quantity() == 0 {
                break;
            }
        }

        // for key in keys_to_remove {
        //     self.asks.remove(&key);
        // }

        return order_matches;
    }

    fn match_order_side_sell(&mut self, result: &mut T) -> Vec<OrderMatch> {
        let mut iter = self.bids.iter_mut();

        let mut order_matches: Vec<OrderMatch> = Vec::new();
        while let Some(next_val) = iter.next() {
            let is_price_match = result.is_match_price(*next_val.0.value());

            // Check if price no match
            if !is_price_match {
                break;
            }

            if is_price_match && next_val.1.total_quantity() > 0 {
                let val = next_val.1.match_order(result);

                order_matches.extend(val);
            }

            if result.quantity() == 0 {
                break;
            }
        }

        return order_matches;
    }

    fn add_order(&mut self, new_order: &T) {
        let last_orders: Option<&mut Orders<T>>;

        if new_order.is_buy() {
            last_orders = self.bids.get_mut(&ReverseOrd::new(new_order.price()));
        } else {
            last_orders = self.asks.get_mut(&new_order.price());
        }

        if last_orders.is_none() {
            let mut orders = Orders::new();
            orders.add(new_order.clone());

            if new_order.is_buy() {
                self.bids.insert(ReverseOrd::new(new_order.price()), orders);
            } else {
                self.asks.insert(new_order.price(), orders);
            }
        } else {
            last_orders.unwrap().add(new_order.clone());
        }
    }
}

impl<T: Order> std::fmt::Display for OrderBook<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = Builder::new();
        builder.push_record(["Bids", "Total", "Asks", "Total"]);

        // Reverse bids for descending order (as bid books are usually displayed)
        let bids: Vec<_> = self.bids.iter().collect();
        let asks: Vec<_> = self.asks.iter().collect();

        let max_len = bids.len().max(asks.len());

        for i in 0..max_len {
            let (bid_price, bid_qty) = bids
                .get(i)
                .map(|(p, o)| {
                    (
                        p.value().to_string(),
                        format!("{}({})", o.total_quantity().to_string(), o.len()),
                    )
                })
                .unwrap_or(("".to_string(), "".to_string()));

            let (ask_price, ask_qty) = asks
                .get(i)
                .map(|(p, o)| {
                    (
                        p.to_string(),
                        format!("{}({})", o.total_quantity().to_string(), o.len()),
                    )
                })
                .unwrap_or(("".to_string(), "".to_string()));

            builder.push_record([bid_price, bid_qty, ask_price, ask_qty]);
        }

        let mut table = builder.build();
        let temp = table.with(Style::markdown());
        write!(f, "{temp}")
    }
}
