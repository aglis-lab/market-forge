use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use market_forge::core::order::OrderSpec;
use rkyv::{Archive, Deserialize, Serialize, util::AlignedVec};
use std::time::Duration;

mod simulate_order;

#[derive(Archive, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct OrderPacket {
    orders: Vec<OrderSpec>,
}

const PACKET_SIZES: [usize; 8] = [10, 50, 75, 100, 150, 1000, 5000, 10000];

fn bench_socket(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (send_socket, listener_task, packets) = rt.block_on(async {
        let socket_path = "./tmp/market_forge.sock";

        let _ = std::fs::remove_file(socket_path);

        let listener = market_forge::network::socket::Listener::bind(socket_path.as_ref())
            .expect("bind failed");

        let listener_task = tokio::spawn(async move {
            let mut socket = listener.accept().await.expect("accept failed");

            loop {
                match socket.recv().await {
                    Ok(packet) => {
                        if packet == b"DONE" {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        let send_socket = market_forge::network::socket::Stream::connect(socket_path.as_ref())
            .await
            .expect("connect failed");

        let packets = convert_packets(&generate_packets(&PACKET_SIZES));

        (send_socket, listener_task, packets)
    });

    let send_socket = std::sync::Arc::new(tokio::sync::Mutex::new(send_socket));

    let mut group = c.benchmark_group("perf_socket");
    group.measurement_time(Duration::from_secs(5));

    for (packet_size, packet) in PACKET_SIZES.iter().zip(&packets) {
        group.throughput(Throughput::Elements(*packet_size as u64));

        let packet = packet.clone();
        let socket = send_socket.clone();

        group.bench_with_input(
            BenchmarkId::from_parameter(packet_size),
            packet_size,
            |b, _| {
                b.to_async(&rt).iter(|| {
                    let socket = socket.clone();
                    let packet = packet.clone();

                    async move {
                        let mut socket = socket.lock().await;

                        socket.send(packet.as_slice()).await.expect("send failed");
                    }
                });
            },
        );
    }

    group.finish();

    rt.block_on(async {
        let mut socket = send_socket.lock().await;

        socket.send(b"DONE").await.unwrap();

        drop(socket);

        listener_task.await.unwrap();
    });
}

fn generate_packets(packet_sizes: &[usize]) -> Vec<OrderPacket> {
    packet_sizes
        .iter()
        .map(|&size| OrderPacket {
            orders: simulate_order::make_realistic_orders(size, 10, size as u64),
        })
        .collect()
}

fn convert_packets(packets: &[OrderPacket]) -> Vec<AlignedVec> {
    packets
        .iter()
        .map(|packet| rkyv::to_bytes::<rkyv::rancor::Error>(packet).expect("serialization failed"))
        .collect()
}

criterion_group!(benches, bench_socket);
criterion_main!(benches);
