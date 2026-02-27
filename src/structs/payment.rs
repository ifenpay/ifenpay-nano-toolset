use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePaymentRequestApi{
    pub receive_address: String,
    pub amount: String,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentResponseApi{
    pub receive_address: String,
    pub amount: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPaymentnApi {
    pub success: bool,
    pub message: String,
    pub transaction_id: String,
    pub is_paid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PaymentStatusRequestApi {
    pub transaction_id: String,
}