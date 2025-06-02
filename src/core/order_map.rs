use std::{collections::BTreeMap, fmt::Display};

use crate::{
    core::order::{OrderId, OrderSide, Quantity},
    core::orders::{Orders, SlabIndex},
};

pub struct OrderMap<P> {
    orders: BTreeMap<P, Orders>,
    total_quantity: Quantity,
}

impl<P: Ord + Clone + Display> OrderMap<P> {
    #[inline(always)]
    pub fn total_quantity(&self) -> Quantity {
        self.total_quantity
    }

    #[inline(always)]
    pub fn set_total_quantity(&mut self, quantity: Quantity) {
        self.total_quantity = quantity;
    }
}

impl<P: Ord + Clone + Display> OrderMap<P> {
    #[inline(always)]
    pub fn new() -> Self {
        return OrderMap {
            orders: BTreeMap::new(),
            total_quantity: 0,
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
    pub fn add_order(
        &mut self,
        key: &P,
        order_idx: SlabIndex,
        order_id: OrderId,
        quantity: Quantity,
    ) {
        self.orders
            .entry(key.clone())
            .or_insert_with(Orders::new)
            .add(order_idx, order_id, quantity);

        self.total_quantity += quantity;
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
    pub fn remove_orders(&mut self, key: &P) -> Option<Orders> {
        self.orders.remove(key)
    }

    #[inline(always)]
    pub fn peek_key(&self) -> Option<&P> {
        self.orders.keys().next()
    }

    #[inline(always)]
    pub fn peek_mut(&mut self) -> Option<(&P, &mut Orders)> {
        self.orders.iter_mut().next()
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<(&P, &Orders)> {
        self.orders.iter().next()
    }

    #[inline(always)]
    pub fn collect_quantity_match_price(
        &self,
        key: &P,
        order_side: &OrderSide,
        quantity: &Quantity,
    ) -> Quantity {
        let mut result: Quantity = 0;

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
    pub fn recalculate_total(&self) -> Quantity {
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
