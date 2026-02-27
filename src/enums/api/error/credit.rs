use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreditError {
    InvalidCreditsAmount
}

impl CreditError {
    pub fn error(&self) -> &'static str {
        match self {
            CreditError::InvalidCreditsAmount => "INVALID_CREDITS_AMOUNT",
        }
    }

    pub fn message(&self) -> String {
        match self {
            CreditError::InvalidCreditsAmount => "Invalid credits amount. Amount must be one of the following (in Nano): 10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000".to_string(),
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