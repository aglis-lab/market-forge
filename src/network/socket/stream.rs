use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Stream {
    inner: tokio::net::UnixStream,
}

impl Stream {
    pub fn new(inner: tokio::net::UnixStream) -> Self {
        Stream { inner }
    }

    pub async fn connect(path: &str) -> anyhow::Result<Self> {
        let inner = tokio::net::UnixStream::connect(path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to socket: {}", e))?;
        Ok(Stream { inner })
    }

    /// Read one length-prefixed packet and deserialize it as `T`.
    pub async fn recv(&mut self) -> anyhow::Result<std::vec::Vec<u8>> {
        let mut len_bytes = [0u8; 4];
        self.inner
            .read_exact(&mut len_bytes)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read length prefix: {}", e))?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        let mut buf = vec![0u8; len];
        self.inner
            .read_exact(&mut buf)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read packet data: {}", e))?;

        Ok(buf)
    }

    /// Write one length-prefixed packet by serializing `data`.
    pub async fn send(&mut self, data: &[u8]) -> anyhow::Result<()> {
        let len = data.len() as u32;
        self.inner.write_all(&len.to_be_bytes()).await?;
        self.inner.write_all(&data).await?;
        self.inner.flush().await?;

        Ok(())
    }
}
