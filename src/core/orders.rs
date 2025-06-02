use std::collections::VecDeque;

use crate::core::order::{OrderId, Quantity};

pub type SlabIndex = u32;

#[derive(Debug, Clone, Copy)]
pub struct OrderMeta {
    slab_idx: SlabIndex,
    order_id: OrderId,
}

pub struct Orders {
    items: VecDeque<OrderMeta>,
    orders_quantity: Quantity,
}

impl OrderMeta {
    pub fn new(slab_idx: SlabIndex, order_id: OrderId) -> Self {
        return Self { order_id, slab_idx };
    }

    pub fn slab_idx(&self) -> SlabIndex {
        self.slab_idx
    }

    pub fn order_id(&self) -> OrderId {
        self.order_id
    }
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
    pub fn add(&mut self, order_idx: SlabIndex, order_id: OrderId, quantity: Quantity) {
        self.items.push_back(OrderMeta::new(order_idx, order_id));
        self.orders_quantity += quantity;
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        return self.items.len() as u32;
    }

    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<OrderMeta> {
        if let Some(front) = self.items.pop_front() {
            return Some(front);
        }

        return None;
    }

    #[inline(always)]
    pub fn items(&self) -> &VecDeque<OrderMeta> {
        &self.items
    }

    #[inline(always)]
    pub fn items_mut(&mut self) -> &mut VecDeque<OrderMeta> {
        &mut self.items
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
