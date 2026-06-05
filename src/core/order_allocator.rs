use core::fmt;

use crate::core::order::{self, Order};
use tabled::{builder::Builder, settings::Style};

const DEFAULT_CAPACITY: usize = 1024;

pub type AllocatorIndex = u64;

// TODO: Use OrderNode to used with orderqueue
pub struct OrderNode<T> {
    order: T,
    prev_idx: Option<AllocatorIndex>,
    next_idx: Option<AllocatorIndex>,
}

pub struct OrderAllocator<T>
where
    T: Order,
{
    inner: slab::Slab<OrderNode<T>>,
    inner_map: rustc_hash::FxHashMap<order::OrderId, AllocatorIndex>,
}

impl<T> OrderNode<T>
where
    T: Order,
{
    #[inline(always)]
    pub fn new(order: T) -> Self {
        return Self {
            order,
            prev_idx: None,
            next_idx: None,
        };
    }

    #[inline(always)]
    pub fn order(&self) -> &T {
        return &self.order;
    }

    #[inline(always)]
    pub fn order_mut(&mut self) -> &mut T {
        return &mut self.order;
    }

    #[inline(always)]
    pub fn prev_idx(&self) -> Option<AllocatorIndex> {
        return self.prev_idx;
    }

    #[inline(always)]
    pub fn set_prev_idx(&mut self, idx: Option<AllocatorIndex>) {
        self.prev_idx = idx;
    }

    #[inline(always)]
    pub fn next_idx(&self) -> Option<AllocatorIndex> {
        return self.next_idx;
    }

    #[inline(always)]
    pub fn set_next_idx(&mut self, idx: Option<AllocatorIndex>) {
        self.next_idx = idx;
    }
}

impl<T> OrderAllocator<T>
where
    T: Order,
{
    #[inline(always)]
    pub fn new() -> Self {
        return OrderAllocator {
            inner: slab::Slab::new(),
            inner_map: rustc_hash::FxHashMap::default(),
        };
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        return OrderAllocator {
            inner: slab::Slab::with_capacity(capacity),
            inner_map: rustc_hash::FxHashMap::with_capacity_and_hasher(
                capacity,
                Default::default(),
            ),
        };
    }

    pub fn default() -> Self {
        return Self::with_capacity(DEFAULT_CAPACITY);
    }

    #[inline(always)]
    pub fn insert(&mut self, order: OrderNode<T>) -> AllocatorIndex {
        let order_id = order.order().id();
        let idx = self.inner.insert(order) as AllocatorIndex;
        self.inner_map.insert(order_id, idx);
        return idx;
    }

    #[inline(always)]
    pub fn get(&self, idx: AllocatorIndex) -> Option<&OrderNode<T>> {
        return self.inner.get(idx as usize);
    }

    #[inline(always)]
    pub fn get_by_order_id(
        &self,
        order_id: order::OrderId,
    ) -> Option<(AllocatorIndex, &OrderNode<T>)> {
        if let Some(&idx) = self.inner_map.get(&order_id) {
            if let Some(order_node) = self.get(idx) {
                return Some((idx, order_node));
            }
        }

        None
    }

    #[inline(always)]
    pub fn get_mut(&mut self, idx: AllocatorIndex) -> Option<&mut OrderNode<T>> {
        return self.inner.get_mut(idx as usize);
    }

    #[inline(always)]
    pub fn try_remove(&mut self, idx: AllocatorIndex) -> Option<OrderNode<T>> {
        if let Some(order_node) = self.inner.try_remove(idx as usize) {
            self.inner_map.remove(&order_node.order().id());
            return Some(order_node);
        }

        None
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.inner.len();
    }

    #[inline(always)]
    pub fn contains(&self, idx: AllocatorIndex) -> bool {
        return self.inner.contains(idx as usize);
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
        self.inner_map.clear();
    }

    #[inline(always)]
    pub fn get2_mut(
        &mut self,
        idx1: AllocatorIndex,
        idx2: AllocatorIndex,
    ) -> Option<(&mut OrderNode<T>, &mut OrderNode<T>)> {
        return self.inner.get2_mut(idx1 as usize, idx2 as usize);
    }
}

impl<T> fmt::Display for OrderAllocator<T>
where
    T: Order + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "OrderAllocator: {} orders", self.inner.len())?;

        let mut builder = Builder::new();
        builder.push_record(["Index", "Order ID", "Order"]);

        for (idx, order_node) in self.inner.iter() {
            builder.push_record([
                idx.to_string(),
                order_node.order().id().to_string(),
                format!("{:?}", order_node.order()),
            ]);
        }

        let mut table = builder.build();
        let temp = table.with(Style::modern_rounded());
        write!(f, "{}", temp)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::order_spec::OrderSpec;

    use super::*;
    use std::mem::size_of;

    #[test]
    fn show_order_node_size() {
        let size = size_of::<OrderNode<OrderSpec>>();
        // Print size; run tests with `-- --nocapture` to see this output.
        println!("OrderNode<OrderSpec> size: {} bytes", size);
        assert!(size > 0);
    }
}
