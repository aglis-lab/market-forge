use std::{collections, fmt};

use crate::core::{order, order_allocator, order_queue};

pub struct OrderMap<P> {
    orders: collections::BTreeMap<P, order_queue::OrderQueue>,
    total_quantity: order::Quantity,
}

impl<P: Ord + Clone + fmt::Display> OrderMap<P> {
    #[inline(always)]
    pub fn total_quantity(&self) -> order::Quantity {
        self.total_quantity
    }

    #[inline(always)]
    pub fn set_total_quantity(&mut self, quantity: order::Quantity) {
        self.total_quantity = quantity;
    }
}

impl<P: Ord + Clone + fmt::Display> OrderMap<P> {
    #[inline(always)]
    pub fn new() -> Self {
        return OrderMap {
            orders: collections::BTreeMap::new(),
            total_quantity: 0,
        };
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.orders.len()
    }

    #[inline(always)]
    pub fn orders(&self) -> &collections::BTreeMap<P, order_queue::OrderQueue> {
        return &self.orders;
    }

    #[inline(always)]
    pub fn add_order(
        &mut self,
        key: &P,
        order_idx: order_allocator::AllocatorIndex,
        order_id: order::OrderId,
        quantity: order::Quantity,
    ) {
        self.orders
            .entry(key.clone())
            .or_insert_with(order_queue::OrderQueue::new)
            .add(order_idx, order_id, quantity);

        self.total_quantity += quantity;
    }

    #[inline(always)]
    pub fn get_orders(&self, key: &P) -> Option<&order_queue::OrderQueue> {
        self.orders.get(key)
    }

    #[inline(always)]
    pub fn get_orders_mut(&mut self, key: &P) -> Option<&mut order_queue::OrderQueue> {
        self.orders.get_mut(key)
    }

    #[inline(always)]
    pub fn remove_orders(&mut self, key: &P) -> Option<order_queue::OrderQueue> {
        self.orders.remove(key)
    }

    #[inline(always)]
    pub fn peek_key(&self) -> Option<&P> {
        self.orders.keys().next()
    }

    #[inline(always)]
    pub fn peek_mut(&mut self) -> Option<(&P, &mut order_queue::OrderQueue)> {
        self.orders.iter_mut().next()
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<(&P, &order_queue::OrderQueue)> {
        self.orders.iter().next()
    }

    #[inline(always)]
    pub fn collect_quantity_match_price(
        &self,
        key: &P,
        order_side: &order::OrderSide,
        quantity: &order::Quantity,
    ) -> order::Quantity {
        let mut result: order::Quantity = 0;

        for (top_price, orders) in self.orders.iter() {
            if (order_side.is_buy() && key >= top_price)
                || (order_side.is_sell() && key <= top_price)
            {
                result += orders.orders_quantity();
            } else {
                break;
            }

            if result > *quantity {
                break;
            }
        }

        return result;
    }

    // For debugging/validation only
    #[inline(always)]
    pub fn recalculate_total(&self) -> order::Quantity {
        self.orders.iter().map(|(_, o)| o.orders_quantity()).sum()
    }

    // Optional: Validate cache consistency
    #[inline(always)]
    pub fn validate_cache(&self) -> Result<(), String> {
        let calculated = self.recalculate_total();
        if self.total_quantity != calculated {
            return Err(format!(
                "Cache inconsistency: cached={}, calculated={}",
                self.total_quantity, calculated
            ));
        }

        Ok(())
    }

    #[inline(always)]
    pub fn collect_until_key<F>(&self, is_break: F) -> Vec<P>
    where
        F: Fn(&P) -> bool,
    {
        let mut result = Vec::new();
        for element in self.orders.keys() {
            if is_break(element) {
                break;
            }

            result.push(element.clone());
        }
        result
    }
}
