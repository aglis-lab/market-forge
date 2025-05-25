use std::{cmp, collections::VecDeque};

use crate::{
    order::{Order, Quantity},
    order_match::OrderMatch,
};

pub struct Orders<T: Order> {
    items: VecDeque<T>,
    total_quantity: Quantity,
}

impl<T: Order> Orders<T> {
    pub fn new() -> Self {
        return Orders {
            items: VecDeque::new(),
            total_quantity: 0,
        };
    }

    pub fn add(&mut self, new_oder: T) {
        let quantity = new_oder.quantity();

        self.items.push_back(new_oder);
        self.total_quantity += quantity;
    }

    pub fn len(&self) -> u32 {
        return self.items.len() as u32;
    }

    pub fn total_quantity(&self) -> Quantity {
        self.total_quantity
    }

    pub fn items(&self) -> &VecDeque<T> {
        &self.items
    }

    pub fn match_order(&mut self, result: &mut T) -> Vec<OrderMatch> {
        // Set Total Quantity
        let min_quantity = cmp::min(self.total_quantity, result.quantity());
        self.total_quantity -= min_quantity;

        // Match through queue
        let mut matches: Vec<OrderMatch> = Vec::new();
        while let Some(front) = self.items.front_mut() {
            let order_match = front.match_order(result);

            if result.quantity() >= front.quantity() {
                self.items.pop_front();
            } else {
                front.set_quantity(front.quantity() - result.quantity());
            }

            matches.push(order_match);

            if result.quantity() == 0 {
                break;
            }
        }

        return matches;
    }
}
