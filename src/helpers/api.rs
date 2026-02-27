use axum::Json;

use crate::structs::{api::ApiResponse};

pub fn api_success<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    })
}