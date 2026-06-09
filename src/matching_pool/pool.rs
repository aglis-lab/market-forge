use crate::{
    core::{order, order_book},
    matching_pool::{MatchingPoolConfig, Symbol},
};
use rtrb::chunks::ChunkError::TooFewSlots;

const BUFFER_CAPACITY: usize = 1024 * 1024 * 16; // 16MB proven safe with high throughput
const INITIAL_POOL_SIZE: usize = 512; // Start with capacity for 512 symbols, can grow dynamically

pub struct MatchingPool<T>
where
    T: order::Order + Send + Sync + 'static,
{
    producers: std::vec::Vec<Option<rtrb::Producer<T>>>,
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

    pub fn get_producer(&mut self, slot_idx: usize) -> Option<&mut rtrb::Producer<T>> {
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
        let (producer, consumer) = rtrb::RingBuffer::<T>::new(BUFFER_CAPACITY);

        // Spin up consumer task for this symbol
        log::info!("Spinning consumer for symbol: {:?}", symbol);

        // Spawn consumer task
        self.spawn_consumer(consumer);

        // Store producer in the pool
        self.store_producer(symbol.slot_idx, producer);
    }

    fn spawn_consumer(&mut self, mut consumer: rtrb::Consumer<T>) {
        self.handles.spawn(async move {
            let mut book = order_book::OrderBook::<T>::default();

            loop {
                let mut chunk = consumer.read_chunk(100);
                if let Err(TooFewSlots(slots)) = chunk {
                    if slots > 0 {
                        chunk = consumer.read_chunk(slots);
                    }
                }

                match chunk {
                    Ok(items) => {
                        for order in items {
                            book.insert_order(&order);
                        }
                    }
                    Err(_) => {
                        if consumer.is_abandoned() {
                            log::info!("Consumer detected abandoned buffer, exiting");
                            break;
                        }

                        tokio::task::yield_now().await;
                        continue;
                    }
                }
            }
        });
    }

    fn store_producer(&mut self, slot_idx: usize, producer: rtrb::Producer<T>) {
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

    use std::{mem::size_of, num};

    #[test]
    fn show_order_node_size() {
        num::NonZeroUsize::new(1).unwrap();

        let size = size_of::<Option<rtrb::Producer<OrderSpec>>>();
        println!("Option<rtrb::Producer<OrderSpec>>> size: {} bytes", size);

        let size = size_of::<Option<rtrb::Consumer<OrderSpec>>>();
        println!("Option<rtrb::Consumer<OrderSpec>>> size: {} bytes", size);

        let size = size_of::<JoinHandle<()>>();
        println!("JoinHandle<()> size: {} bytes", size);

        let size = size_of::<Option<JoinHandle<()>>>();
        println!("Option<JoinHandle<()>> size: {} bytes", size);

        let size = size_of::<tokio::task::JoinSet<()>>();
        println!("JoinSet<()> size: {} bytes", size);

        assert!(size > 0);
    }
}
