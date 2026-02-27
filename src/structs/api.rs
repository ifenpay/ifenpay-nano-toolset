use axum::Json;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub type ApiErrorResult = (StatusCode, Json<ApiResponse<ApiError>>);
pub type ApiSuccessResult<T> = Json<ApiResponse<T>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}