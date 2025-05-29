use std::collections::VecDeque;

use crate::order::Quantity;

pub struct Orders {
    items: VecDeque<usize>,
    orders_quantity: Quantity,
}

impl Orders {
    #[inline(always)]
    pub fn new() -> Self {
        return Orders {
            items: VecDeque::new(),
            orders_quantity: 0,
        };
    }

    #[inline(always)]
    pub fn add(&mut self, order_idx: usize, quantity: Quantity) {
        self.items.push_back(order_idx);
        self.orders_quantity += quantity;
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        return self.items.len() as u32;
    }

    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<usize> {
        if let Some(front) = self.items.pop_front() {
            return Some(front);
        }

        return None;
    }

    #[inline(always)]
    pub fn items(&self) -> &VecDeque<usize> {
        &self.items
    }

    #[inline(always)]
    pub fn orders_quantity(&self) -> Quantity {
        self.orders_quantity
    }

    #[inline(always)]
    pub fn set_orders_quantity(&mut self, quantity: Quantity) {
        self.orders_quantity = quantity;
    }
}
