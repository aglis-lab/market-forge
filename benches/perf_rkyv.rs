use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use market_forge::core::order::OrderSpec;
use rkyv::{Archive, Deserialize, Serialize};
use std::time::Duration;

mod simulate_order;

#[derive(Archive, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct OrderPacket {
    orders: Vec<OrderSpec>,
}

// 1000 packets at once is best for high throughput
const PACKET_SIZES: [usize; 7] = [10, 50, 100, 150, 1000, 5000, 10000];

fn generate_packets(packet_sizes: &[usize]) -> Vec<OrderPacket> {
    packet_sizes
        .iter()
        .map(|&size| OrderPacket {
            orders: simulate_order::make_realistic_orders(size, 10, size as u64),
        })
        .collect()
}

fn bench_serialize(c: &mut Criterion) {
    let packets = generate_packets(&PACKET_SIZES);

    let mut group = c.benchmark_group("rkyv_serialize");
    group.measurement_time(Duration::from_secs(5));

    for (packet_size, packet) in PACKET_SIZES.iter().zip(packets.iter()) {
        group.throughput(Throughput::Elements(*packet_size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(packet_size),
            packet,
            |b, packet| {
                b.iter(|| {
                    let _ = rkyv::to_bytes::<rkyv::rancor::Error>(packet).unwrap();
                });
            },
        );
    }

    group.finish();
}

fn bench_deserialize(c: &mut Criterion) {
    let packets = generate_packets(&PACKET_SIZES);

    // Pre-serialize outside benchmark.
    let serialized_packets: Vec<_> = packets
        .iter()
        .map(|packet| rkyv::to_bytes::<rkyv::rancor::Error>(packet).unwrap())
        .collect();

    let mut group = c.benchmark_group("rkyv_deserialize");
    group.measurement_time(Duration::from_secs(5));

    for (packet_size, bytes) in PACKET_SIZES.iter().zip(serialized_packets.iter()) {
        group.throughput(Throughput::Elements(*packet_size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(packet_size),
            bytes,
            |b, bytes| {
                b.iter(|| {
                    let _: OrderPacket =
                        rkyv::from_bytes::<OrderPacket, rkyv::rancor::Error>(bytes).unwrap();
                });
            },
        );
    }

    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let packets = generate_packets(&PACKET_SIZES);

    let mut group = c.benchmark_group("rkyv_roundtrip");
    group.measurement_time(Duration::from_secs(5));

    for (packet_size, packet) in PACKET_SIZES.iter().zip(packets.iter()) {
        group.throughput(Throughput::Elements(*packet_size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(packet_size),
            packet,
            |b, packet| {
                b.iter(|| {
                    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(packet).unwrap();

                    let _: OrderPacket =
                        rkyv::from_bytes::<OrderPacket, rkyv::rancor::Error>(&bytes).unwrap();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_serialize, bench_deserialize, bench_roundtrip);

criterion_main!(benches);
