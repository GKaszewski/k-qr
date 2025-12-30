use crate::domain::error::ConfigError;
use config::Config as Configuration;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub qr_default_color: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let builder = Configuration::builder()
            .set_default("server_host", "0.0.0.0")
            .map_err(|e| ConfigError::InvalidOption(e.to_string()))?
            .set_default("server_port", 3000)
            .map_err(|e| ConfigError::InvalidInteger(e.to_string()))?
            .set_default("qr_default_color", "#000000")
            .map_err(|e| ConfigError::InvalidOption(e.to_string()))?
            .add_source(config::Environment::default());

        builder
            .build()
            .map_err(|e| ConfigError::EnvVarMissing(e.to_string()))?
            .try_deserialize()
            .map_err(|e| ConfigError::InvalidOption(e.to_string()))
    }
}
