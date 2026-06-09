use crate::{
    core::{order, order_book},
    matching_pool::{MatchingPoolConfig, Symbol},
};
use async_ringbuf::{
    AsyncHeapRb, AsyncRb,
    traits::{AsyncConsumer, AsyncProducer},
    wrap::AsyncWrap,
};
use ringbuf::{
    storage::Heap,
    traits::{Consumer, Producer, Split},
};
use std::sync::Arc;

const BUFFER_CAPACITY: usize = 1024 * 1024 * 4; // 16MB proven safe with high throughput
const INITIAL_POOL_SIZE: usize = 512;

type ConsumerBuffer<T> = AsyncWrap<Arc<AsyncRb<Heap<T>>>, false, true>;
type ProducerBuffer<T> = AsyncWrap<Arc<AsyncRb<Heap<T>>>, true, false>;

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

    // TODO: I have an idea to push failed orders to a fallback buffer for later processing, but let's start with this for now
    // The logic is we push failed order into a vector
    // and then wake up a background task to process the failed orders in batches, which can be more efficient than processing them one by one
    // all orders with same symbol_id can be processed in the same batch, which can further improve efficiency
    pub fn try_push(&mut self, slot_idx: usize, order: T) -> anyhow::Result<()> {
        if let Some(producer) = self.get_producer(slot_idx) {
            return producer
                .try_push(order)
                .map_err(|e| anyhow::anyhow!("Failed to push order: {:?}", e));
        } else {
            Err(anyhow::anyhow!(
                "Attempted to push to non-existent symbol with slot_idx: {}",
                slot_idx
            ))
        }
    }

    pub async fn push(&mut self, slot_idx: usize, order: T) -> anyhow::Result<()> {
        if let Some(producer) = self.get_producer(slot_idx) {
            let result = producer.push(order).await;
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Attempted to push to non-existent symbol with slot_idx: {}",
                slot_idx
            ))
        }
    }

    fn get_producer(&mut self, slot_idx: usize) -> Option<&mut ProducerBuffer<T>> {
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
        let (producer, consumer) = AsyncHeapRb::<T>::new(BUFFER_CAPACITY).split();

        // Spin up consumer task for this symbol
        log::info!("Spinning consumer for symbol: {:?}", symbol);

        // Spawn consumer task
        self.spawn_consumer(consumer);

        // Store producer in the pool
        self.store_producer(symbol.slot_idx, producer);
    }

    fn spawn_consumer(&mut self, mut consumer: ConsumerBuffer<T>) {
        self.handles.spawn(async move {
            let mut book = order_book::OrderBook::<T>::default();

            loop {
                // Wake up when there's an order to process
                match consumer.pop().await {
                    Some(order) => book.insert_order(&order),
                    None => {
                        log::info!("Consumer received None, exiting consumer task");
                        break;
                    }
                };

                // Process all available orders in the buffer
                loop {
                    let slices = consumer.occupied_slices();
                    let count = slices.0.len() + slices.1.len();
                    if count == 0 {
                        break;
                    }

                    for slot in slices.0.iter().chain(slices.1.iter()) {
                        let order = unsafe { slot.assume_init_ref() };
                        book.insert_order(order);
                    }

                    unsafe {
                        consumer.advance_read_index(count);
                    };
                }
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
