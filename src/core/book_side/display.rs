use core::fmt;

use crate::core::{book_side::BookSide, order};

impl<P: Ord + Clone + fmt::Display> BookSide<P> {}
