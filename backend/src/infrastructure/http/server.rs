use crate::application::generate_qr::GenerateQrUseCase;
use crate::ports::HttpServer;
use async_trait::async_trait;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;

use super::handlers::{generate, index};

pub struct AxumServer {
    use_case: Arc<GenerateQrUseCase>,
}

impl AxumServer {
    pub fn new(use_case: Arc<GenerateQrUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl HttpServer for AxumServer {
    async fn run(&self, host: &str, port: u16) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/", get(index))
            .route("/api/qr", get(generate))
            .with_state(self.use_case.clone());

        let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
        tracing::info!("Server listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}
