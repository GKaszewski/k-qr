use super::error::QrError;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct QrData(String);

impl QrData {
    pub fn new(data: impl Into<String>) -> Result<Self, QrError> {
        let data = data.into();
        if data.trim().is_empty() {
            return Err(QrError::EmptyData);
        }
        if data.len() > 2048 {
            return Err(QrError::DataTooLong(data.len()));
        }
        Ok(Self(data))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Newtype for validated hex color codes.
#[derive(Debug, Clone, PartialEq)]
pub struct HexColor(String);

impl HexColor {
    pub fn new(color: impl Into<String>) -> Result<Self, QrError> {
        let color = color.into();
        let trimmed = color.trim();

        // Must start with # and be 7 chars (#RRGGBB) or 4 chars (#RGB)
        if !trimmed.starts_with('#') {
            return Err(QrError::InvalidOption(format!(
                "Color must start with #: {}",
                color
            )));
        }

        let hex_part = &trimmed[1..];
        if hex_part.len() != 6 && hex_part.len() != 3 {
            return Err(QrError::InvalidOption(format!(
                "Invalid color length: {}",
                color
            )));
        }

        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(QrError::InvalidOption(format!(
                "Invalid hex characters: {}",
                color
            )));
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for HexColor {
    fn default() -> Self {
        Self("#000000".to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QrOptions {
    pub color: HexColor,
}

impl Default for QrOptions {
    fn default() -> Self {
        Self {
            color: HexColor::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_data_valid() {
        let input = "https://example.com";
        let data = QrData::new(input);
        assert!(data.is_ok());
        assert_eq!(data.unwrap().as_str(), input);
    }

    #[test]
    fn test_qr_data_empty() {
        let input = "   ";
        let data = QrData::new(input);
        assert_eq!(data, Err(QrError::EmptyData));
    }

    #[test]
    fn test_qr_data_too_long() {
        let input = "a".repeat(2049);
        let data = QrData::new(&input);
        match data {
            Err(QrError::DataTooLong(len)) => assert_eq!(len, 2049),
            _ => panic!("Expected DataTooLong error"),
        }
    }

    #[test]
    fn test_hex_color_valid() {
        assert!(HexColor::new("#000000").is_ok());
        assert!(HexColor::new("#fff").is_ok());
        assert!(HexColor::new("#E57E1D").is_ok());
    }

    #[test]
    fn test_hex_color_invalid() {
        assert!(HexColor::new("000000").is_err()); // Missing #
        assert!(HexColor::new("#gggggg").is_err()); // Invalid chars
        assert!(HexColor::new("#12345").is_err()); // Wrong length
    }
}
