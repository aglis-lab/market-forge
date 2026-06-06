use std::num::NonZeroU32;

pub struct NonZeroSlab<T> {
    inner: slab::Slab<T>,
}

impl<T> NonZeroSlab<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            inner: slab::Slab::new(),
        }
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: slab::Slab::with_capacity(capacity),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline(always)]
    pub fn contains(&self, key: NonZeroU32) -> bool {
        self.inner.contains(Self::convert_from_nonzero_u32(key))
    }

    #[inline(always)]
    pub fn insert(&mut self, val: T) -> NonZeroU32 {
        Self::convert_to_nonzero_u32(self.inner.insert(val))
    }

    #[inline(always)]
    pub fn get(&self, key: NonZeroU32) -> Option<&T> {
        self.inner.get(Self::convert_from_nonzero_u32(key))
    }

    #[inline(always)]
    pub fn get_mut(&mut self, key: NonZeroU32) -> Option<&mut T> {
        self.inner.get_mut(Self::convert_from_nonzero_u32(key))
    }

    #[inline(always)]
    pub fn get2_mut(&mut self, key1: NonZeroU32, key2: NonZeroU32) -> Option<(&mut T, &mut T)> {
        self.inner.get2_mut(
            Self::convert_from_nonzero_u32(key1),
            Self::convert_from_nonzero_u32(key2),
        )
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = (NonZeroU32, &T)> {
        self.inner
            .iter()
            .map(|(idx, val)| (Self::convert_to_nonzero_u32(idx), val))
    }

    #[inline(always)]
    pub fn remove(&mut self, key: NonZeroU32) -> Option<T> {
        self.inner.try_remove(Self::convert_from_nonzero_u32(key))
    }

    // Utility functions to convert between usize and NonZeroU32
    #[inline(always)]
    fn convert_to_nonzero_u32(raw_idx: usize) -> NonZeroU32 {
        debug_assert!(raw_idx < u32::MAX as usize, "Index exceeds u32 limits");
        unsafe { NonZeroU32::new_unchecked((raw_idx + 1) as u32) }
    }

    // Utility functions to convert between usize and NonZeroU32
    #[inline(always)]
    fn convert_from_nonzero_u32(key: NonZeroU32) -> usize {
        (key.get() - 1) as usize
    }
}
