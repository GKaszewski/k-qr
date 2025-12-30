use crate::domain::{
    error::QrError,
    qr::{QrData, QrOptions},
};
use crate::ports::QrCodeGenerator;
use std::sync::Arc;

pub struct GenerateQrUseCase {
    generator: Arc<dyn QrCodeGenerator>,
}

impl GenerateQrUseCase {
    pub fn new(generator: Arc<dyn QrCodeGenerator>) -> Self {
        Self { generator }
    }

    pub async fn execute(&self, data: String, options: QrOptions) -> Result<Vec<u8>, QrError> {
        let qr_data = QrData::new(data)?;
        self.generator.generate(&qr_data, &options).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MockQrCodeGenerator;

    #[tokio::test]
    async fn test_generate_qr_success() {
        let mut mock_generator = MockQrCodeGenerator::new();
        mock_generator
            .expect_generate()
            .times(1)
            .returning(|_, _| Ok(vec![1, 2, 3]));

        let use_case = GenerateQrUseCase::new(Arc::new(mock_generator));
        let result = use_case
            .execute("https://example.com".to_string(), QrOptions::default())
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_generate_qr_invalid_data() {
        let mock_generator = MockQrCodeGenerator::new();
        let use_case = GenerateQrUseCase::new(Arc::new(mock_generator));

        let result = use_case.execute("".to_string(), QrOptions::default()).await;
        assert!(matches!(result, Err(QrError::EmptyData)));
    }
}
