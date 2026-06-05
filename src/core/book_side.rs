use std::collections;

use crate::core::{order, order_allocator, price_level};

pub struct BookSide<P> {
    pub(super) price_levels: collections::BTreeMap<P, price_level::PriceLevel>,
    pub(super) total_quantity: order::Quantity,
}

impl<P: Ord + Clone> BookSide<P> {
    #[inline(always)]
    pub fn new() -> Self {
        return BookSide {
            price_levels: collections::BTreeMap::new(),
            total_quantity: 0,
        };
    }
}

// Common methods for BookSide
impl<P: Ord + Clone> BookSide<P> {
    // Update total quantity (quantity)
    // Get price level (price)
    //      Not exits, create new price level and add order to price level (price, quantity, allocator_idx)
    //  Update price level quantity (quantity)
    //  Update price level tail (allocator_idx)
    //  Update len (len)
    // Return previous tail (allocator_idx)
    #[inline(always)]
    pub fn upsert_price_level(
        &mut self,
        key: P,
        allocator_idx: order_allocator::AllocatorIndex,
        quantity: order::Quantity,
    ) -> Option<order_allocator::AllocatorIndex> {
        let mut prev_tail = None;

        self.total_quantity += quantity;
        self.price_levels
            .entry(key.clone())
            .and_modify(|price_level| {
                prev_tail = Some(price_level.tail());

                price_level.set_quantity(price_level.quantity() + quantity);
                price_level.set_tail(allocator_idx);
                price_level.set_len(price_level.len() + 1);
            })
            .or_insert_with(|| price_level::PriceLevel::new(allocator_idx, quantity));

        return prev_tail;
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = (&P, &price_level::PriceLevel)> {
        self.price_levels.iter()
    }

    #[inline(always)]
    pub fn get_price_level(&self, key: &P) -> Option<&price_level::PriceLevel> {
        self.price_levels.get(key)
    }

    #[inline(always)]
    pub fn get_price_level_mut(&mut self, key: &P) -> Option<&mut price_level::PriceLevel> {
        self.price_levels.get_mut(key)
    }

    #[inline(always)]
    pub fn remove_price(&mut self, key: &P) -> Option<price_level::PriceLevel> {
        if let Some(price_level) = self.price_levels.remove(key) {
            Some(price_level)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn peek_price(&self) -> Option<&P> {
        self.price_levels.keys().next()
    }

    #[inline(always)]
    pub fn collect_quantity_match_price(
        &self,
        key: &P,
        order_side: &order::OrderSide,
        quantity: &order::Quantity,
    ) -> order::Quantity {
        let mut result: order::Quantity = 0;

        for (top_price, orders) in self.price_levels.iter() {
            if (order_side.is_buy() && key >= top_price)
                || (order_side.is_sell() && key <= top_price)
            {
                result += orders.quantity();
            } else {
                break;
            }

            if result > *quantity {
                break;
            }
        }

        return result;
    }

    #[inline(always)]
    pub fn set_total_quantity(&mut self, new_quantity: order::Quantity) {
        self.total_quantity = new_quantity;
    }

    #[inline(always)]
    pub fn total_quantity(&self) -> order::Quantity {
        self.total_quantity
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.price_levels.len()
    }

    #[inline(always)]
    pub fn recalculate_total(&self) -> order::Quantity {
        self.price_levels.iter().map(|(_, o)| o.quantity()).sum()
    }

    // Optional: Validate cache consistency
    #[inline(always)]
    pub fn validate_cache(&self) -> Result<(), String> {
        let calculated = self.recalculate_total();
        if self.total_quantity != calculated {
            return Err(format!(
                "Cache inconsistency: cached={}, calculated={}",
                self.total_quantity, calculated
            ));
        }

        Ok(())
    }
}
