use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreditError {
    TransactionCreationFailed,
    PriceCalculationError,
    InvalidCreditsAmount,
    TransactionNotConfirmed,
    NoTransactionFound,
    ApiKeyNotFound,
}

impl CreditError {
    pub fn error(&self) -> &'static str {
        match self {
            CreditError::TransactionCreationFailed => "TRANSACTION_CREATION_FAILED", // used
            CreditError::PriceCalculationError => "PRICE_CALCULATION_ERROR", // used
            CreditError::InvalidCreditsAmount => "INVALID_CREDITS_AMOUNT",
            CreditError::TransactionNotConfirmed => "TRANSACTION_NOT_CONFIRMED",
            CreditError::NoTransactionFound => "NO_TRANSACTION_FOUND",
            CreditError::ApiKeyNotFound => "API_KEY_NOT_FOUND",
        }
    }

    pub fn message(&self) -> String {
        match self {
            CreditError::TransactionCreationFailed => "Failed to create top-up transaction".to_string(),
            CreditError::PriceCalculationError => "Failed to calculate credit prices".to_string(),
            CreditError::InvalidCreditsAmount => "Invalid credits amount. Amount must be one of the following (in Nano): 10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000".to_string(),
            CreditError::TransactionNotConfirmed => "The top-up transaction has not been confirmed yet".to_string(),
            CreditError::NoTransactionFound => "No transaction found with the provided ID".to_string(),
            CreditError::ApiKeyNotFound => "API key not found".to_string(),
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