use std::{cmp::Ordering, fmt::Display};

use crate::{order::Price, order_map::OrderMapKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReverseOrd<T: Display>(pub T);

impl OrderMapKey for ReverseOrd<Price> {
    #[inline(always)]
    fn to_price(&self) -> Price {
        self.0
    }
}

impl<T: Display> ReverseOrd<T> {
    #[inline(always)]
    pub fn new(val: T) -> Self {
        return ReverseOrd(val);
    }
}

impl<T: Ord + Display> Ord for ReverseOrd<T> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0) // reverse
    }
}

impl<T: Ord + Display> PartialOrd for ReverseOrd<T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Display> Display for ReverseOrd<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}
