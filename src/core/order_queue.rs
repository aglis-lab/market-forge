use crate::core::{order, order_allocator};
use std::collections;

const CAPACITY: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct OrderMeta {
    allocator_idx: order_allocator::AllocatorIndex,
    order_id: order::OrderId,
}

pub struct OrderQueue {
    // The queue of orders at a specific price level, ordered by time priority (FIFO).
    queue: collections::VecDeque<OrderMeta>,
    // The total quantity of all orders in this queue, used for quick access to the total quantity at this price level.
    total_quantity: order::Quantity,
}

impl OrderMeta {
    pub fn new(allocator_idx: order_allocator::AllocatorIndex, order_id: order::OrderId) -> Self {
        return Self {
            order_id,
            allocator_idx,
        };
    }

    pub fn allocator_idx(&self) -> order_allocator::AllocatorIndex {
        self.allocator_idx
    }

    pub fn order_id(&self) -> order::OrderId {
        self.order_id
    }
}

impl OrderQueue {
    #[inline(always)]
    pub fn new() -> Self {
        return OrderQueue {
            total_quantity: 0,
            queue: collections::VecDeque::with_capacity(CAPACITY),
        };
    }

    #[inline(always)]
    pub fn add(
        &mut self,
        allocator_idx: order_allocator::AllocatorIndex,
        order_id: order::OrderId,
        quantity: order::Quantity,
    ) {
        self.queue
            .push_back(OrderMeta::new(allocator_idx, order_id));
        self.total_quantity += quantity;
    }

    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<OrderMeta> {
        self.queue.pop_front()
    }

    #[inline(always)]
    pub fn remove_index(&mut self, index: usize) -> Option<OrderMeta> {
        if index >= self.queue.len() {
            return None;
        }

        Some(self.queue.remove(index).unwrap())
    }

    // TODO: We can optimize it by using prev and next indices inside order allocator, but for simplicity, we will use binary search here.
    // With those optimizations, we can achieve O(1) complexity for finding and removing orders, but it will require more complex data structures and management.
    #[inline(always)]
    pub fn find_order(&self, order_id: order::OrderId) -> Option<(usize, OrderMeta)> {
        return self
            .queue
            .iter()
            .enumerate()
            .find(|(_, meta)| meta.order_id() == order_id)
            .map(|(index, meta)| (index, *meta));
    }

    #[inline(always)]
    pub fn peek_front(&self) -> Option<&OrderMeta> {
        self.queue.front()
    }

    #[inline(always)]
    pub fn orders_quantity(&self) -> order::Quantity {
        self.total_quantity
    }

    #[inline(always)]
    pub fn set_orders_quantity(&mut self, quantity: order::Quantity) {
        self.total_quantity = quantity;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn order_meta_size() {
        println!("Size of OrderMeta: {} bytes", size_of::<OrderMeta>());
        assert_eq!(size_of::<OrderMeta>(), 16usize);
    }
}
