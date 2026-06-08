use crate::{
    core::order,
    matching_pool::{MatchingPoolConfig, Symbol},
};
use async_ringbuf::traits::AsyncConsumer;
use async_ringbuf::{AsyncHeapRb, traits::Split, wrap::AsyncWrap};
use std::sync::Arc;

const BUFFER_CAPACITY: usize = 1024 * 1024; // 1MB proven safe with high throughput
const INITIAL_POOL_SIZE: usize = 1024;

type ConsumerBuffer<T> = AsyncWrap<Arc<AsyncHeapRb<T>>, false, true>;
type ProducerBuffer<T> = AsyncWrap<Arc<AsyncHeapRb<T>>, true, false>;

pub struct MatchingPool<T>
where
    T: order::Order + Send + Sync + 'static,
{
    producers: std::vec::Vec<Option<ProducerBuffer<T>>>,
    handles: tokio::task::JoinSet<()>,
}

impl<T> MatchingPool<T>
where
    T: order::Order + Send + Sync + 'static,
{
    pub fn new() -> Self {
        let producers = (0..INITIAL_POOL_SIZE).map(|_| None).collect();

        Self {
            producers: producers,
            handles: tokio::task::JoinSet::new(),
        }
    }

    pub fn init(&mut self, config: MatchingPoolConfig) {
        for symbol in config.symbols.iter() {
            self.create_ring_buffer(symbol)
        }
    }

    pub fn get_producer(&mut self, slot_idx: usize) -> Option<&mut ProducerBuffer<T>> {
        if slot_idx >= self.producers.len() {
            log::warn!(
                "Attempted to get producer for non-existent symbol with slot_idx: {}",
                slot_idx
            );

            return None;
        }

        return self.producers[slot_idx].as_mut();
    }

    pub fn cancel(&mut self, slot_idx: usize) {
        if slot_idx >= self.producers.len() {
            log::warn!(
                "Attempted to cancel non-existent symbol with slot_idx: {}",
                slot_idx
            );
            return;
        }

        if self.producers[slot_idx].is_none() {
            log::warn!(
                "Attempted to cancel symbol with slot_idx: {} that has no producer",
                slot_idx
            );
            return;
        }

        self.producers[slot_idx] = None;
    }

    pub async fn wait_all(&mut self) {
        while let Some(handle) = self.handles.join_next().await {
            match handle {
                Ok(_) => log::info!("Consumer task completed successfully"),
                Err(e) => log::error!("Consumer task failed: {}", e),
            }
        }
    }

    fn create_ring_buffer(&mut self, symbol: &Symbol) {
        let (producer, consumer) = async_ringbuf::AsyncHeapRb::<T>::new(BUFFER_CAPACITY).split();

        // Spin up consumer task for this symbol
        log::info!("Spinning consumer for symbol: {:?}", symbol);

        // Spawn consumer task
        self.spawn_consumer(consumer);

        // Store producer in the pool
        self.store_producer(symbol.slot_idx, producer);
    }

    fn spawn_consumer(&mut self, mut consumer: ConsumerBuffer<T>) {
        self.handles.spawn(async move {
            loop {
                let _ = match consumer.pop().await {
                    Some(packet) => packet,
                    None => {
                        log::info!("Consumer detected closed buffer, exiting");
                        return;
                    }
                };
            }
        });
    }

    fn store_producer(&mut self, slot_idx: usize, producer: ProducerBuffer<T>) {
        if slot_idx >= self.producers.len() {
            self.producers.resize_with(slot_idx + 1, || None);
        }

        self.producers[slot_idx] = Some(producer);
    }
}

#[cfg(test)]
mod tests {
    use tokio::task::JoinHandle;

    use crate::core::order::OrderSpec;

    use super::*;
    use std::{mem::size_of, num};

    #[test]
    fn show_order_node_size() {
        num::NonZeroUsize::new(1).unwrap();

        let size = size_of::<Option<ProducerBuffer<OrderSpec>>>();
        println!(
            "Option<ProducerBuffer<PacketOrders<OrderSpec>>>> size: {} bytes",
            size
        );

        let size = size_of::<Option<ConsumerBuffer<OrderSpec>>>();
        println!(
            "Option<ConsumerBuffer<PacketOrders<OrderSpec>>>> size: {} bytes",
            size
        );

        let size = size_of::<JoinHandle<()>>();
        println!("JoinHandle<()> size: {} bytes", size);

        let size = size_of::<Option<JoinHandle<()>>>();
        println!("Option<JoinHandle<()>> size: {} bytes", size);

        let size = size_of::<tokio::task::JoinSet<()>>();
        println!("JoinSet<()> size: {} bytes", size);

        assert!(size > 0);
    }
}
