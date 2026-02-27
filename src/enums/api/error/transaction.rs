use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionError {
    InvalidTransactionId,
    InvalidNegativeAmount,
    InvalidNumberFormat,
    InvalidWholeNumber,
    TooManyDecimalPlaces,
    InvalidFractionalPart,
    AmountToLarge,
    InsufficientFunds,
}

impl TransactionError {
    pub fn error(&self) -> &'static str {
        match self {
            TransactionError::InvalidTransactionId => "INVALID_TRANSACTION_ID", // used
            TransactionError::InvalidNegativeAmount => "INVALID_NEGATIVE_AMOUNT", // used
            TransactionError::InvalidNumberFormat => "INVALID_NUMBER_FORMAT", // used
            TransactionError::InvalidWholeNumber => "INVALID_WHOLE_NUMBER", // used
            TransactionError::TooManyDecimalPlaces => "TOO_MANY_DECIMAL_PLACES", // used
            TransactionError::InvalidFractionalPart => "INVALID_FRACTIONAL_PART", // used
            TransactionError::AmountToLarge => "AMOUNT_TOO_LARGE", // used
            TransactionError::InsufficientFunds => "INSUFFICIENT_FUNDS", // used
        }
    }

    pub fn message(&self) -> String {
        match self {
            TransactionError::InvalidTransactionId => "The provided transaction ID is invalid".to_string(),
            TransactionError::InvalidNegativeAmount => "The provided amount is negative".to_string(),
            TransactionError::InvalidNumberFormat => "The provided number format is invalid".to_string(),
            TransactionError::InvalidWholeNumber => "The provided whole number is invalid".to_string(),
            TransactionError::TooManyDecimalPlaces => "The provided amount has too many decimal places (maximum is 30)".to_string(),
            TransactionError::InvalidFractionalPart => "The provided fractional part is invalid".to_string(),
            TransactionError::AmountToLarge => "The provided amount is too large".to_string(),
            TransactionError::InsufficientFunds => "Insufficient funds for this transaction".to_string(),
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