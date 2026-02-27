use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockError {
    KeyDerivationFailed,
    InvalidPreviousHash,
    InvalidRepresentativePublicKey,
    InvalidLink,
    BlockHashGenerationFailed,
    SigningFailed,
    InvalidRepresentativeAddress,
    InvalidWorkHex,
    InvalidWorkLength,
    InvalidWorkRoot,
    InvalidWorkRootLength,
    CalculateWorkFailed,
}

impl BlockError {
    pub fn error(&self) -> &'static str {
        match self {
            BlockError::KeyDerivationFailed => "KEY_DERIVATION_FAILED",
            BlockError::InvalidPreviousHash => "INVALID_PREVIOUS_HASH",
            BlockError::InvalidRepresentativePublicKey => "INVALID_REPRESENTATIVE_PUBLIC_KEY",
            BlockError::InvalidLink => "INVALID_LINK",
            BlockError::BlockHashGenerationFailed => "BLOCK_HASH_GENERATION_FAILED",
            BlockError::SigningFailed => "SIGNING_FAILED",
            BlockError::InvalidRepresentativeAddress => "INVALID_REPRESENTATIVE_ADDRESS",
            BlockError::InvalidWorkHex => "INVALID_WORK_HEX",
            BlockError::InvalidWorkLength => "INVALID_WORK_LENGTH",
            BlockError::InvalidWorkRoot => "INVALID_WORK_ROOT",
            BlockError::InvalidWorkRootLength => "INVALID_WORK_ROOT_LENGTH",
            BlockError::CalculateWorkFailed => "CALCULATE_WORK_FAILED",
        }
    }

    pub fn message(&self) -> String {
        match self {
            BlockError::KeyDerivationFailed => "Failed to derive key from seed and index".to_string(),
            BlockError::InvalidPreviousHash => "Invalid previous hash".to_string(),
            BlockError::InvalidRepresentativePublicKey => "Invalid representative public key".to_string(),
            BlockError::InvalidLink => "Invalid link".to_string(),
            BlockError::BlockHashGenerationFailed => "Failed to generate block hash".to_string(),
            BlockError::SigningFailed => "Failed to sign block hash".to_string(),
            BlockError::InvalidRepresentativeAddress => "Invalid representative address".to_string(),
            BlockError::InvalidWorkHex => "Invalid work hex".to_string(),
            BlockError::InvalidWorkLength => "Work must be exactly 8 bytes".to_string(),
            BlockError::InvalidWorkRoot => "Invalid work root".to_string(),
            BlockError::InvalidWorkRootLength => "Work root must be exactly 32 bytes".to_string(),
            BlockError::CalculateWorkFailed => "Failed to calculate work value".to_string(),
        }
    }

    pub fn to_response(self) -> ApiErrorResult {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse { 
                success: false, 
                data: None, 
                error: Some(ApiError {
                    error: self.error().to_string(),
                    message: self.message(),
                }),
            })
        )
    }
}