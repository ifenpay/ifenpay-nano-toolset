use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountError {
    InvalidAddress,
}

impl AccountError {
    pub fn error(&self) -> &'static str {
        match self {
            AccountError::InvalidAddress => "INVALID_ADDRESS", // used
        }
    }

    pub fn message(&self) -> String {
        match self {
            AccountError::InvalidAddress => "Address must start with 'nano_' or 'xrb_'".to_string(),
        }
    }

    pub fn to_response(self) -> ApiErrorResult {
        (
            StatusCode::BAD_REQUEST,
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