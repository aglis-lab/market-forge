use std::{cmp, collections::VecDeque};

use crate::{
    order::{self, Order, Quantity},
    order_match::OrderMatch,
};

pub struct Orders {
    items: VecDeque<usize>,
    total_quantity: Quantity,
}

impl Orders {
    pub fn new() -> Self {
        return Orders {
            items: VecDeque::new(),
            total_quantity: 0,
        };
    }

    pub fn add(&mut self, order_idx: usize, quantity: Quantity) {
        self.items.push_back(order_idx);
        self.total_quantity += quantity;
    }

    pub fn len(&self) -> u32 {
        return self.items.len() as u32;
    }

    pub fn total_quantity(&self) -> Quantity {
        self.total_quantity
    }

    pub fn pop_front(&mut self) -> Option<usize> {
        if let Some(front) = self.items.pop_front() {
            return Some(front);
        }

        return None;
    }

    pub fn items(&self) -> &VecDeque<usize> {
        &self.items
    }

    pub fn set_total_quantity(&mut self, quantity: Quantity) {
        self.total_quantity = quantity;
    }

    // Match order with the queue
    // This will match the order with the front of the queue
    // and return a vector of OrderMatch
    // pub fn match_order(&mut self, result: &mut T) -> Vec<OrderMatch> {
    //     // Set Total Quantity
    //     let min_quantity = cmp::min(self.total_quantity, result.quantity());
    //     self.total_quantity -= min_quantity;

    //     // Match through queue
    //     let mut matches: Vec<OrderMatch> = Vec::new();
    //     while let Some(front) = self.items.front_mut() {
    //         let match_order = front.match_order(result);

    //         // Push match order
    //         matches.push(match_order);

    //         // Check if we need to remove front order
    //         if front.quantity() == 0 {
    //             self.items.pop_front();
    //         }

    //         // If result order is fully matched, we can break
    //         if result.quantity() == 0 {
    //             break;
    //         }
    //     }

    //     return matches;
    // }
}
