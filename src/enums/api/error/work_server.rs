use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkServerError {
    Error(String),
}

impl WorkServerError {
    pub fn error(&self) -> &'static str {
        match self {
            WorkServerError::Error(_) => "WORK_SERVER_ERROR",
        }
    }

    pub fn message(&self) -> String {
        match self {
            WorkServerError::Error(msg) => format!("Work server error: {}", msg),
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