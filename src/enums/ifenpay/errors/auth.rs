use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthError {
    Forbidden,
    InvalidApiKey,
    MissingApiKey,
    PayloadTooLarge,
    RateLimitExceeded,
}

impl AuthError {
    pub fn error(&self) -> &'static str {
        match self {
            AuthError::Forbidden => "FORBIDDEN",
            AuthError::InvalidApiKey => "INVALID_API_KEY", // used
            AuthError::MissingApiKey => "MISSING_API_KEY", // used
            AuthError::PayloadTooLarge => "PAYLOAD_TOO_LARGE", // used
            AuthError::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
        }
    }

    pub fn message(&self) -> String {
        match self {
            AuthError::Forbidden => "Access to this resource is forbidden".to_string(),
            AuthError::InvalidApiKey => "The provided API key is invalid".to_string(),
            AuthError::MissingApiKey => "API key is missing from the request".to_string(),
            AuthError::PayloadTooLarge => "Request payload exceeds the maximum allowed size of 5 KB.".to_string(),
            AuthError::RateLimitExceeded => "Rate limit exceeded. Please try again later.".to_string(),
        }
    }

    pub fn to_response(self) -> ApiErrorResult {
        (
            StatusCode::UNAUTHORIZED,
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