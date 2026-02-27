use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfoResponseApi {
    pub block_account: String,
    pub amount: String,
    pub balance: String,
    pub height: String,
    pub local_timestamp: String,
    pub confirmed: String,
    pub contents: StateBlock,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateBlock {
    #[serde(rename = "type")]
    pub block_type: String, // Always "state"
    pub account: String,
    pub previous: String,
    pub representative: String,
    pub balance: String,
    pub link: String,
    pub signature: String,
    pub work: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedBlock {
    pub block: StateBlock,
    pub hash: String,
    pub account_public_key: String,
    pub subtype: String,
    pub work_root: String,
    pub threshold_hex: String,
    pub work_value_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBlockRequestApi {
    pub subtype: String,
    pub block: StateBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBlockResponseApi {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateWorkResponseApi {
    pub work: String,
    pub difficulty: String,
    pub multiplier: String,
}