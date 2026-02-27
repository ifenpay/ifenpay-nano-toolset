use axum::Json;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::structs::api::api::{ApiError, ApiErrorResult, ApiResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebsocketError {
    WebSocketAddAccountError,
    WebSocketRemoveAccountError,
    WebSocketNotInitialized,
}

impl WebsocketError {
    pub fn error(&self) -> &'static str {
        match self {
            WebsocketError::WebSocketAddAccountError => "WEBSOCKET_ADD_ACCOUNT_ERROR", // used
            WebsocketError::WebSocketRemoveAccountError => "WEBSOCKET_REMOVE_ACCOUNT_ERROR", // used
            WebsocketError::WebSocketNotInitialized => "WEBSOCKET_NOT_INITIALIZED",
        }
    }

    pub fn message(&self) -> String {
        match self {
            WebsocketError::WebSocketAddAccountError => format!("Failed to send add account command"),
            WebsocketError::WebSocketRemoveAccountError => format!("Failed to send remove account command").to_string(),
            WebsocketError::WebSocketNotInitialized => "WebSocket command sender not initialized".to_string(),
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