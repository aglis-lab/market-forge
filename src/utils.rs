use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReverseOrd<T>(T);

impl<T> ReverseOrd<T> {
    pub fn new(val: T) -> Self {
        return ReverseOrd(val);
    }

    pub fn value(&self) -> &T {
        return &self.0;
    }
}

impl<T: Ord> Ord for ReverseOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0) // reverse
    }
}

impl<T: Ord> PartialOrd for ReverseOrd<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
