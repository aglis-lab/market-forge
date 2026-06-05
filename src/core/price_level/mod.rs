use core::fmt;

use crate::core::{order, order_allocator};

// Represents a price level in the order book
// Head <-|--|--|--|--|--|--|--|--|--|-> Tail
#[derive(Debug, Clone)]
pub struct PriceLevel {
    head: order_allocator::AllocatorIndex,
    tail: order_allocator::AllocatorIndex,
    len: usize,
    quantity: order::Quantity,
}

impl PriceLevel {
    #[inline(always)]
    pub fn new(
        allocator_index: order_allocator::AllocatorIndex,
        quantity: order::Quantity,
    ) -> Self {
        return PriceLevel {
            head: allocator_index,
            tail: allocator_index,
            len: 1,
            quantity: quantity,
        };
    }
}

impl PriceLevel {
    #[inline(always)]
    pub fn set_head(&mut self, head: order_allocator::AllocatorIndex) {
        self.head = head;
    }

    #[inline(always)]
    pub fn head(&self) -> order_allocator::AllocatorIndex {
        self.head
    }

    #[inline(always)]
    pub fn set_tail(&mut self, tail: order_allocator::AllocatorIndex) {
        self.tail = tail;
    }

    #[inline(always)]
    pub fn tail(&self) -> order_allocator::AllocatorIndex {
        self.tail
    }

    #[inline(always)]
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    #[inline(always)]
    pub fn quantity(&self) -> order::Quantity {
        self.quantity
    }

    #[inline(always)]
    pub fn set_quantity(&mut self, quantity: order::Quantity) {
        self.quantity = quantity;
    }
}

impl fmt::Display for PriceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PriceLevel {{ head: {}, tail: {}, len: {}, quantity: {} }}",
            self.head, self.tail, self.len, self.quantity
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn price_level_size() {
        println!("Size of PriceLevel: {} bytes", size_of::<PriceLevel>());
    }
}
