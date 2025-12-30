use crate::application::generate_qr::GenerateQrUseCase;
use crate::domain::qr::QrOptions;
use crate::infrastructure::http::views::{index_page, qr_component};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn index() -> impl IntoResponse {
    index_page()
}

pub async fn generate(
    State(use_case): State<Arc<GenerateQrUseCase>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let data = params.get("data").cloned().unwrap_or_default();

    // Simple options for now, can be extended to parse from params
    let options = QrOptions::default();

    match use_case.execute(data, options).await {
        Ok(image_data) => qr_component(&image_data).into_response(),
        Err(e) => format!("<div class='error'>Error: {}</div>", e).into_response(),
    }
}
