use std::{cmp::min, collections::BTreeMap};

use callback::Callback;
use tabled::{builder::Builder, settings::Style};

use crate::order::{self, OrderSide, Orders, Price};
mod callback;

pub struct OrderBook {
    bids: BTreeMap<Price, Orders>,
    asks: BTreeMap<Price, Orders>,

    callback: Option<Callback>,
}

impl OrderBook {
    pub fn new(callback: Option<Callback>) -> Self {
        return OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            callback: callback,
        };
    }

    pub fn add(&mut self, new_order: &order::Order) {
        if new_order.quantity() == 0 {
            return;
        }

        if new_order.is_buy() {
            Self::match_order(&mut self.bids, new_order);
        } else {
            Self::match_order(&mut self.asks, new_order);
        }
    }

    fn match_order(book: &mut BTreeMap<Price, Orders>, new_order: &order::Order) -> bool {
        let price = new_order.price();
        let mut book_iter = book.iter();
        // while let Some(last_order) = book_iter.next() {
        //     if new_order.match_price(last_order.0.clone()) {

        //     }
        // }

        let last_orders = book.get_mut(&price);
        if last_orders.is_none() {
            let mut orders = Orders::new();

            orders.add(new_order.clone());
            book.insert(price, orders);
        } else {
            last_orders.unwrap().add(new_order.clone());
        }

        return true;
    }
}

impl std::fmt::Display for OrderBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = Builder::new();
        builder.push_record(["Bids", "Total", "Asks", "Total"]);

        // Reverse bids for descending order (as bid books are usually displayed)
        let bids: Vec<_> = self.bids.iter().rev().collect();
        let asks: Vec<_> = self.asks.iter().collect();

        let max_len = bids.len().max(asks.len());

        for i in 0..max_len {
            let (bid_price, bid_qty) = bids
                .get(i)
                .map(|(p, o)| (p.to_string(), o.total_quantity().to_string()))
                .unwrap_or(("".to_string(), "".to_string()));
            let (ask_price, ask_qty) = asks
                .get(i)
                .map(|(p, o)| (p.to_string(), o.total_quantity().to_string()))
                .unwrap_or(("".to_string(), "".to_string()));

            builder.push_record([bid_price, bid_qty, ask_price, ask_qty]);
        }

        let mut table = builder.build();
        let temp = table.with(Style::markdown());
        write!(f, "{temp}")
    }
}
