use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletData {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub wallet_private_seed: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponseApi {
    pub account: String,
    pub balance: String, // In Nano
    pub balance_raw: String,
    pub pending: String, // In Nano
    pub pending_raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksPerAccountResponse {
    pub blocks: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfoResponseApi {
    pub frontier: String,
    pub balance: String,
    pub representative: String,
    pub difficulty_send: String,
    pub difficulty_receive: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SendNanoRequestApi {
    pub recipient_address: String,
    pub amount: String, // In Nano
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendNanoResponseApi{
    pub amount: String,
    pub recipient: String, // In Nano
}