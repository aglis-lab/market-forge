use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrderError {
    #[error("Orders not found")]
    OrdersNotFound,

    #[error("Order not found")]
    OrderNotFound,

    #[error("Order not found at slab")]
    SlabOrderNotFound,

    #[error("Failed to remove order from slab")]
    SlabFailedRemoveOrder,

    // Insert Order
    #[error("No order was match")]
    NoOrderMatch,
}
