use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

use crate::{
    order::{Price, Quantity},
    orders::Orders,
};

pub struct OrderMap<P> {
    order_price: BinaryHeap<P>,
    order_price_exist: HashSet<P>,
    orders: HashMap<Price, Orders>,
}

impl<P: Ord + Hash + Clone> OrderMap<P> {
    pub fn new() -> Self {
        return OrderMap {
            order_price: BinaryHeap::new(),
            order_price_exist: HashSet::new(),
            orders: HashMap::new(),
        };
    }

    pub fn len(&self) -> usize {
        self.order_price.len()
    }

    pub fn order_prices(&self) -> &BinaryHeap<P> {
        return &self.order_price;
    }

    pub fn orders(&self) -> &HashMap<Price, Orders> {
        return &self.orders;
    }

    pub fn add_order(&mut self, key: &P, price: Price, order_idx: usize, quantity: Quantity) {
        self.orders
            .entry(price)
            .or_insert_with(Orders::new)
            .add(order_idx, quantity);

        if !self.order_price_exist.contains(key) {
            self.order_price.push(key.clone());
            self.order_price_exist.insert(key.clone());
        }
    }

    pub fn get_orders(&self, price: &Price) -> Option<&Orders> {
        self.orders.get(price)
    }

    pub fn get_orders_mut(&mut self, price: &Price) -> Option<&mut Orders> {
        self.orders.get_mut(price)
    }

    pub fn remove_order(&mut self, key: &P, price: &Price) -> Option<Orders> {
        assert!(
            key.clone()
                == self
                    .order_price
                    .peek()
                    .expect("order_price peek not found")
                    .clone(),
            "key and peek order price should be equal"
        );

        self.order_price_exist.remove(key);
        self.order_price.pop();
        self.orders.remove(price)
    }

    pub fn peek(&self) -> Option<&P> {
        self.order_price.peek()
    }
}
