use crate::domain::{
    error::QrError,
    qr::{QrData, QrOptions},
};
use async_trait::async_trait;

/// Port for QR code generation
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait QrCodeGenerator: Send + Sync {
    async fn generate(&self, data: &QrData, options: &QrOptions) -> Result<Vec<u8>, QrError>;
}

/// Port for HTTP server - abstracts the web framework
#[async_trait]
pub trait HttpServer: Send + Sync {
    async fn run(&self, host: &str, port: u16) -> anyhow::Result<()>;
}
