use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Environment variable not found: {0}")]
    EnvVarMissing(String),
    #[error("Invalid integer configuration: {0}")]
    InvalidInteger(String),
    #[error("Invalid option: {0}")]
    InvalidOption(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum QrError {
    #[error("QR Code data cannot be empty")]
    EmptyData,
    #[error("QR Code data is too long: {0} characters")]
    DataTooLong(usize),
    #[error("Failed to generate QR code: {0}")]
    GenerationFailed(String),
    #[error("Invalid option: {0}")]
    InvalidOption(String),
}
