use crate::core::order;

#[derive(Debug, Clone)]
pub struct PacketOrders<T>
where
    T: order::Order,
{
    pub orders: Vec<T>,
}
