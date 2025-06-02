use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum OrderError {
    #[error("Orders not found")]
    OrdersNotFound,

    #[error("Order not found")]
    OrderNotFound,

    #[error("Order not found at slab")]
    SlabOrderNotFound,

    #[error("Failed to remove order from slab")]
    SlabFailedRemoveOrder,

    #[error("No order was match")]
    NoOrderMatch,

    #[error("Order already filled")]
    OrderAlreadyFilled,
}
