use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum  DatabaseError {
    SelectError,
    StoreError,
    FailedToUpdate,
}


impl DatabaseError {
    pub fn error(&self) -> &'static str {
        match self {
            DatabaseError::SelectError => "DATABASE_SELECT_ERROR", // used
            DatabaseError::StoreError => "DATABASE_ERROR", // used
            DatabaseError::FailedToUpdate => "DATABASE_UPDATE_ERROR",
        }
    }

    pub fn message(&self) -> String {
        match self {
            DatabaseError::SelectError => "Failed to retrieve data from the database".to_string(),
            DatabaseError::StoreError => "Failed to store data in the database".to_string(),
            DatabaseError::FailedToUpdate => "Failed to update data in the database".to_string(),

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