use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use k_qr::application::generate_qr::GenerateQrUseCase;
use k_qr::infrastructure::{
    config::AppConfig, http::server::AxumServer, qr_adapter::QrCodeAdapter,
};
use k_qr::ports::HttpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "k_qr=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::load()?;
    tracing::info!("Configuration loaded: {:?}", config);

    let qr_adapter = Arc::new(QrCodeAdapter);
    let generate_use_case = Arc::new(GenerateQrUseCase::new(qr_adapter));

    let server: Box<dyn HttpServer> = Box::new(AxumServer::new(generate_use_case));

    server.run(&config.server_host, config.server_port).await?;

    Ok(())
}
