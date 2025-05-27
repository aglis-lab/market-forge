use std::{collections::BTreeMap, fmt::Display, hash::Hash};

use crate::{order::Quantity, orders::Orders};

pub struct OrderMap<P> {
    orders: BTreeMap<P, Orders>,
}

impl<P: Ord + Hash + Clone + Display> OrderMap<P> {
    #[inline(always)]
    pub fn new() -> Self {
        return OrderMap {
            orders: BTreeMap::new(),
        };
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.orders.len()
    }

    #[inline(always)]
    pub fn orders(&self) -> &BTreeMap<P, Orders> {
        return &self.orders;
    }

    #[inline(always)]
    pub fn add_order(&mut self, key: &P, order_idx: usize, quantity: Quantity) {
        self.orders
            .entry(key.clone())
            .or_insert_with(Orders::new)
            .add(order_idx, quantity);
    }

    #[inline(always)]
    pub fn get_orders(&self, key: &P) -> Option<&Orders> {
        self.orders.get(key)
    }

    #[inline(always)]
    pub fn get_orders_mut(&mut self, key: &P) -> Option<&mut Orders> {
        self.orders.get_mut(key)
    }

    #[inline(always)]
    pub fn remove_order(&mut self, key: &P) -> Option<Orders> {
        self.orders.remove(key)
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<&P> {
        self.orders.keys().next()
    }
}
