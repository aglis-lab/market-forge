use std::collections::BTreeMap;

use crate::{order::Order, orders::Orders};
pub struct OrderMap<Key, T: Order> {
    items: BTreeMap<Key, Orders<T>>,
}

impl<Key: Ord, T: Order> OrderMap<Key, T> {
    pub fn new() -> Self {
        return OrderMap {
            items: BTreeMap::new(),
        };
    }

    pub fn get(&mut self, key: &Key) -> Option<&mut Orders<T>> {
        return self.items.get_mut(key);
    }

    pub fn insert(&mut self, key: Key, value: Orders<T>) {
        self.items.insert(key, value);
    }

    pub fn items(&self) -> &BTreeMap<Key, Orders<T>> {
        return &self.items;
    }

    pub fn items_mut(&mut self) -> &mut BTreeMap<Key, Orders<T>> {
        return &mut self.items;
    }
}
