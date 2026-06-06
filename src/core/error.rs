use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum OrderError {
    #[error("Price level not found")]
    PriceLevelNotFound,

    #[error("Order not found")]
    OrderNotFound,

    #[error("Order next index not found when trying to cancel order")]
    OrderNextIdxNotFound,

    #[error("Order previous index not found when trying to cancel order")]
    OrderPrevIdxNotFound,

    #[error("Invalid quantity delta")]
    InvalidQuantityDelta,

    #[error("Order not found at slab")]
    SlabOrderNotFound,

    #[error("Failed to remove order from slab")]
    SlabFailedRemoveOrder,
}
