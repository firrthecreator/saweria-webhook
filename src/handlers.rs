use axum::{
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use tracing::info;
use crate::models::{SaweriaDonation, ApiResponse};

/// Handles the incoming webhook request from Saweria.
///
/// It deserializes the JSON payload into `SaweriaDonation`.
/// If the data format is incorrect, Axum will automatically reject it (422).
pub async fn handle_webhook(
    Json(payload): Json<SaweriaDonation>,
) -> impl IntoResponse {
    // Log the donation details clearly to the console
    info!("   Donation Received!");
    info!("   From: {}", payload.donator_name);
    info!("   Amount: Rp {}", payload.amount);
    // Use unwrap_or to handle cases where the message is missing
    info!("   Message: {}", payload.message.as_deref().unwrap_or("-"));

    // Business Logic:
    // TODO: Add your custom logic here (e.g., save to DB, forward to Discord/OBS).
    
    // Construct a success response
    let response = ApiResponse {
        status: "success".to_string(),
        message: "Donation processed successfully".to_string(),
    };

    // Return 200 OK to Saweria
    (StatusCode::OK, Json(response))
}

/// Simple health check to verify the service is up.
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Saweria Webhook Service is Running")
}
