use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DonateResponseApi {
    pub success: bool,
    pub amount: String, // In Nano
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DonateAddressResponse {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DonateRequestApi {
    pub amount: String,
}