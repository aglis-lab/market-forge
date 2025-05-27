use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
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

impl<P: Ord + Hash + Clone + Display> OrderMap<P> {
    #[inline(always)]
    pub fn new() -> Self {
        return OrderMap {
            order_price: BinaryHeap::new(),
            order_price_exist: HashSet::new(),
            orders: HashMap::new(),
        };
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.order_price.len()
    }

    #[inline(always)]
    pub fn order_prices(&self) -> &BinaryHeap<P> {
        return &self.order_price;
    }

    #[inline(always)]
    pub fn orders(&self) -> &HashMap<Price, Orders> {
        return &self.orders;
    }

    #[inline(always)]
    pub fn add_order(&mut self, key: &P, order_idx: usize, price: Price, quantity: Quantity) {
        self.orders
            .entry(price)
            .or_insert_with(Orders::new)
            .add(order_idx, quantity);

        self.add_key(key);
    }

    #[inline(always)]
    pub fn get_orders(&self, price: &Price) -> Option<&Orders> {
        self.orders.get(price)
    }

    #[inline(always)]
    pub fn get_orders_mut(&mut self, price: &Price) -> Option<&mut Orders> {
        self.orders.get_mut(price)
    }

    #[inline(always)]
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

        self.remove_key(key);
        self.orders.remove(price)
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<&P> {
        self.order_price.peek()
    }

    #[inline(always)]
    fn add_key(&mut self, key: &P) {
        if !self.order_price_exist.contains(key) {
            self.order_price.push(key.clone());
            self.order_price_exist.insert(key.clone());
        }
    }

    #[inline(always)]
    fn remove_key(&mut self, key: &P) {
        assert!(
            self.order_price_exist.contains(key),
            "key should exist in order_price_exist"
        );
        assert!(
            self.order_price.peek().is_some(),
            "order_price should not be empty"
        );
        assert!(
            self.order_price.peek().unwrap() == key,
            "key should be equal to the peek of order_price"
        );

        // Rebuild the heap to maintain the order
        if self.order_price_exist.remove(key) {
            self.order_price.pop();
        }
    }
}
