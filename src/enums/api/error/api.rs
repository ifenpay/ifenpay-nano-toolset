use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiErrorType {
    RequestError,
    ParseError,
    InvalidData,
}

impl ApiErrorType {
    pub fn error(&self) -> &'static str {
        match self {
            ApiErrorType::RequestError => "REQUEST_ERROR",
            ApiErrorType::ParseError => "PARSE_ERROR",
            ApiErrorType::InvalidData => "INVALID_DATA",
        }
    }

    pub fn message(&self) -> String {
        match self {
            ApiErrorType::RequestError => "Request to upstream API failed".to_string(),
            ApiErrorType::ParseError => "Failed to parse upstream response".to_string(),
            ApiErrorType::InvalidData => "Invalid data provided".to_string(),
        }
    }

    pub fn to_response(self) -> ApiErrorResult {
        (
            StatusCode::BAD_GATEWAY,
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