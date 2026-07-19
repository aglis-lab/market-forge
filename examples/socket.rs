use std::vec;
use std::{fs, path::Path};

use market_forge::core::order::OrderSpec;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct Packet {
    orders: Vec<OrderSpec>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let folder = "./tmp";
    if fs::metadata(folder).is_err() {
        fs::create_dir(folder).expect(&format!("Failed to create {} directory", folder));
    }

    let socket_path = format!("{}/market_forge.sock", folder);
    log::debug!("Starting socket example with socket path: {}", socket_path);

    let _ = accept(socket_path.clone()).await;
    let _ = send(socket_path.clone()).await;

    // Keep the main thread alive to allow the socket to accept connections
    loop {
        std::thread::park();
    }
}

async fn accept(socket_path: String) -> anyhow::Result<()> {
    if Path::new(&socket_path).exists() {
        let _ = fs::remove_file(&socket_path);
    }

    tokio::task::spawn(async move {
        let listener = market_forge::network::socket::Listener::bind(socket_path.as_ref())
            .expect("Failed to bind socket listener");

        let mut stream = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        log::debug!("Listening on {}, waiting for connections...", socket_path);

        loop {
            let packet = match stream.recv().await {
                Ok(packet) => rkyv::from_bytes::<Packet, rkyv::rancor::Error>(&packet),
                Err(e) => {
                    log::error!("Connection closed: {}", e);
                    break;
                }
            };

            if let Ok(packet) = packet {
                log::debug!("Received packet: {:?}", packet);
            } else {
                log::error!("Failed to deserialize packet: {}", packet.err().unwrap());
            }
        }
    });

    Ok(())
}

async fn send(socket_path: String) -> anyhow::Result<()> {
    tokio::task::spawn(async move {
        let mut socket = retry(
            || market_forge::network::socket::Stream::connect(socket_path.as_ref()),
            3,
            std::time::Duration::from_secs(1),
        )
        .await
        .expect("Failed to connect the socket");
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));

        loop {
            interval.tick().await;

            let val = vec![
                market_forge::core::order::OrderSpec::limit_price(
                    1,
                    1,
                    market_forge::core::order::OrderSide::Buy,
                    1000,
                    10,
                ),
                market_forge::core::order::OrderSpec::market(
                    1,
                    2,
                    market_forge::core::order::OrderSide::Sell,
                    5,
                ),
                market_forge::core::order::OrderSpec::market(
                    1,
                    2,
                    market_forge::core::order::OrderSide::Sell,
                    5,
                ),
            ];

            let val_bytes = match rkyv::to_bytes::<rkyv::rancor::Error>(&Packet { orders: val }) {
                Ok(bytes) => bytes,
                Err(e) => {
                    log::error!("Failed to serialize packet: {}", e);
                    continue;
                }
            };

            log::debug!("Sending packet to socket stream at {}", socket_path);
            let _ = socket.send(val_bytes.as_slice()).await;
        }
    });

    Ok(())
}

async fn retry<F, T, V>(f: T, retries: usize, delay: std::time::Duration) -> anyhow::Result<V>
where
    T: Fn() -> F,
    F: std::future::Future<Output = anyhow::Result<V>>,
{
    for i in 0..retries {
        match f().await {
            Ok(val) => return Ok(val),
            Err(err) => {
                if i < retries - 1 {
                    log::warn!("Attempt {} failed: {}. Retrying...", i + 1, err);
                    tokio::time::sleep(delay).await;
                } else {
                    return Err(anyhow::anyhow!("All retries failed: {}", err));
                }
            }
        }
    }

    Err(anyhow::anyhow!("All retries failed"))
}
