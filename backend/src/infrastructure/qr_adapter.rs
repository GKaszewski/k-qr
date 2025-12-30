use crate::domain::{
    error::QrError,
    qr::{QrData, QrOptions},
};
use crate::ports::QrCodeGenerator;
use async_trait::async_trait;
use image::Luma;
use qrcode::QrCode;
use std::io::Cursor;

pub struct QrCodeAdapter;

#[async_trait]
impl QrCodeGenerator for QrCodeAdapter {
    async fn generate(&self, data: &QrData, _options: &QrOptions) -> Result<Vec<u8>, QrError> {
        let qr =
            QrCode::new(data.as_str()).map_err(|e| QrError::GenerationFailed(e.to_string()))?;

        let qr_image = qr.render::<Luma<u8>>().build();
        let mut buffer: Vec<u8> = Vec::new();

        qr_image
            .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
            .map_err(|e| QrError::GenerationFailed(e.to_string()))?;

        Ok(buffer)
    }
}
