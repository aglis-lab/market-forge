use crate::core::{order, order_allocator};

pub struct ReplaceOrder {
    pub order_id: order::OrderId,
    pub new_price: order::Price,
    pub quantity_delta: order::SignQuantity,
}

pub(super) struct ProcessMatchResult {
    pub next_idx: Option<order_allocator::AllocatorIndex>,
    pub head_order_remaining: order::Quantity,
    pub matched_quantity: order::Quantity,
}

impl ProcessMatchResult {
    #[inline(always)]
    pub fn is_head_fully_matched(&self) -> bool {
        self.head_order_remaining == 0
    }
}
