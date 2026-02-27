use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcError {
    ReadError,
    ResponseError(String),
    ParseError(String),
    RequestError(String),
}

impl RpcError {
    pub fn error(&self) -> &'static str {
        match self {
            RpcError::ReadError => "RPC_READ_ERROR", // used
            RpcError::ResponseError(_) => "RPC_RESPONSE_ERROR", // used
            RpcError::ParseError(_) => "RPC_PARSE_ERROR", // used
            RpcError::RequestError(_) => "RPC_REQUEST_ERROR", // used
        }
    }

    pub fn message(&self) -> String {
        match self {
            RpcError::ReadError => "Failed to read RPC response".to_string(),
            RpcError::ResponseError(msg) => format!("RPC response indicates an error: {}", msg),
            RpcError::ParseError(details) => format!("Failed to parse RPC response: {}", details),
            RpcError::RequestError(details) => format!("Failed to send RPC request: {}", details),
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