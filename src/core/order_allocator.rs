use crate::core::order::Order;

const DEFAULT_CAPACITY: usize = 1024;

pub type AllocatorIndex = u64;

pub struct OrderAllocator<T>
where
    T: Order,
{
    inner: slab::Slab<T>,
}

impl<T> OrderAllocator<T>
where
    T: Order,
{
    #[inline(always)]
    pub fn new() -> Self {
        return OrderAllocator {
            inner: slab::Slab::new(),
        };
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        return OrderAllocator {
            inner: slab::Slab::with_capacity(capacity),
        };
    }

    pub fn default() -> Self {
        return Self::with_capacity(DEFAULT_CAPACITY);
    }

    #[inline(always)]
    pub fn insert(&mut self, order: T) -> AllocatorIndex {
        return self.inner.insert(order) as AllocatorIndex;
    }

    #[inline(always)]
    pub fn get(&self, idx: AllocatorIndex) -> Option<&T> {
        return self.inner.get(idx as usize);
    }

    #[inline(always)]
    pub fn get_mut(&mut self, idx: AllocatorIndex) -> Option<&mut T> {
        return self.inner.get_mut(idx as usize);
    }

    #[inline(always)]
    pub fn try_remove(&mut self, idx: AllocatorIndex) -> Option<T> {
        return self.inner.try_remove(idx as usize);
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
    }

    #[inline(always)]
    pub fn get2_mut(
        &mut self,
        idx1: AllocatorIndex,
        idx2: AllocatorIndex,
    ) -> Option<(&mut T, &mut T)> {
        return self.inner.get2_mut(idx1 as usize, idx2 as usize);
    }
}
