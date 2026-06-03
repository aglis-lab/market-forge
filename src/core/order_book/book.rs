use crate::{
    core::{order, order_allocator, order_map},
    utils::ReverseOrd,
};

type PriceLevelBids = order_map::OrderMap<ReverseOrd<order::Price>>;
type PriceLevelAsks = order_map::OrderMap<order::Price>;

pub struct OrderBook<T: order::Order> {
    // Memory Allocator
    pub(super) order_allocator: order_allocator::OrderAllocator<T>,

    // Bids and Asks
    pub(super) bids: PriceLevelBids,
    pub(super) asks: PriceLevelAsks,
}

// Instantiate OrderBook
impl<T: order::Order> OrderBook<T> {
    #[inline(always)]
    pub fn new() -> Self {
        return OrderBook {
            order_allocator: order_allocator::OrderAllocator::new(),
            bids: PriceLevelBids::new(),
            asks: PriceLevelAsks::new(),
        };
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        return OrderBook {
            order_allocator: order_allocator::OrderAllocator::with_capacity(capacity),
            asks: PriceLevelAsks::new(),
            bids: PriceLevelBids::new(),
        };
    }

    #[inline(always)]
    pub fn default() -> Self {
        return OrderBook {
            order_allocator: order_allocator::OrderAllocator::default(),
            asks: PriceLevelAsks::new(),
            bids: PriceLevelBids::new(),
        };
    }

    // Optional: Validate cache consistency
    #[inline(always)]
    pub fn validate_cache(&self) -> Result<(), String> {
        self.asks.validate_cache()?;
        self.bids.validate_cache()?;

        return Ok(());
    }
}

// OrderBook common Methods, use only for parent module
impl<T: order::Order> OrderBook<T> {
    #[inline(always)]
    pub(super) fn peek_top_price(&self, is_bids: bool) -> Option<&order::Price> {
        if is_bids {
            return self.bids.peek_key().map(|i| &i.0);
        } else {
            return self.asks.peek_key();
        }
    }

    #[inline(always)]
    pub(super) fn has_sufficient_quantity(&self, order: &T) -> bool {
        let quantity: order::Quantity = {
            if order.is_limit_price() {
                if order.is_buy() {
                    self.asks.collect_quantity_match_price(
                        &order.price(),
                        &order.order_side(),
                        &order.quantity(),
                    )
                } else {
                    self.bids.collect_quantity_match_price(
                        &ReverseOrd::new(order.price()),
                        &order.order_side(),
                        &order.quantity(),
                    )
                }
            } else {
                // Market Order
                if order.is_buy() {
                    self.asks.total_quantity()
                } else {
                    self.bids.total_quantity()
                }
            }
        };

        return quantity >= order.quantity();
    }

    #[inline(always)]
    pub(super) fn is_match_price(
        &self,
        order_side: &order::OrderSide,
        order_price: order::Price,
        top_price: order::Price,
    ) -> bool {
        if order_side.is_buy() && order_price >= top_price {
            return true;
        }

        if order_side.is_sell() && order_price <= top_price {
            return true;
        }

        false
    }

    #[inline(always)]
    pub(super) fn remove_orders(&mut self, is_bids: bool, top_price: &order::Price) {
        if is_bids {
            self.bids.remove_orders(&ReverseOrd::new(*top_price));
        } else {
            self.asks.remove_orders(&top_price);
        }
    }

    #[inline(always)]
    pub(super) fn set_total_quantity(&mut self, is_bids: bool, new_quantity: order::Quantity) {
        if is_bids {
            self.bids.set_total_quantity(new_quantity);
        } else {
            self.asks.set_total_quantity(new_quantity);
        }
    }

    #[inline(always)]
    pub(super) fn decrease_total_quantity(&mut self, is_bids: bool, quantity: order::Quantity) {
        if is_bids {
            self.set_total_quantity(is_bids, self.bids.total_quantity() - quantity);
        } else {
            self.set_total_quantity(is_bids, self.asks.total_quantity() - quantity);
        }
    }
}
