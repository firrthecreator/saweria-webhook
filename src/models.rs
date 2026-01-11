use serde::{Deserialize, Serialize};

/// Represents the payload sent by Saweria webhook.
/// Mapped to match the production JSON structure.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SaweriaDonation {
    /// Unique identifier for the donation transaction.
    pub id: String,

    /// The gross amount donated (before tax/cut).
    /// Mapped from "amount_raw" in the JSON payload.
    #[serde(rename = "amount_raw")]
    pub amount: f64,

    /// Name of the donator.
    #[serde(rename = "donator_name")]
    pub donator_name: String,

    /// Email of the donator.
    /// Wrapped in Option because anonymous users might not provide it.
    #[serde(rename = "donator_email")]
    pub donator_email: Option<String>,

    /// The message attached to the donation.
    /// Default to None if the field is missing.
    #[serde(default)]
    pub message: Option<String>,

    /// Timestamp of the donation.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

/// Standard API response returned to Saweria.
#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}
