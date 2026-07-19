use super::Stream;

pub struct Listener {
    inner: tokio::net::UnixListener,
}

impl Listener {
    pub fn bind(path: &str) -> anyhow::Result<Self> {
        let inner = tokio::net::UnixListener::bind(path)
            .map_err(|e| anyhow::anyhow!("Failed to bind to socket: {}", e))?;
        Ok(Listener { inner })
    }

    pub async fn accept(&self) -> anyhow::Result<Stream> {
        let (stream, _) = self
            .inner
            .accept()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to accept connection: {}", e))?;
        Ok(Stream::new(stream))
    }
}
