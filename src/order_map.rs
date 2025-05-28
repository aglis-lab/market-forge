use std::{cmp, collections::BTreeMap, fmt::Display, hash::Hash};

use crate::{
    order::{OrderSide, Price, Quantity},
    order_match::MatchPrices,
    orders::Orders,
};

pub trait OrderMapKey: Ord + Clone + Display + Hash {
    fn to_price(&self) -> Price;
}

pub struct OrderMap<P> {
    orders: BTreeMap<P, Orders>,
}

impl<P: OrderMapKey> OrderMap<P> {
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

    #[inline(always)]
    pub fn collect_match_price_quantity(
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
                result += orders.total_quantity();
            } else {
                break;
            }

            if result > quantity.clone() {
                break;
            }
        }

        return result;
    }

    #[inline(always)]
    pub fn collect_match_prices(
        &self,
        key: &P,
        order_side: &OrderSide,
        quantity: &Quantity,
    ) -> MatchPrices {
        let mut result = MatchPrices::new();
        let mut quantity = quantity.clone();
        for (top_price, orders) in self.orders.iter() {
            if (order_side.is_buy() && key >= top_price)
                || (order_side.is_sell() && key <= top_price)
            {
                result.get_top_prices_mut().push(top_price.to_price());
                quantity -= cmp::min(quantity, orders.total_quantity());
            } else {
                break;
            }

            if quantity <= 0 {
                break;
            }
        }

        if quantity > 0 {
            result.set_partially_match();
        }

        return result;
    }
}
