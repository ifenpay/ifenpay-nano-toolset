use std::env;

use axum::Json;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::enums::api::error::api::ApiErrorType;
use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};

pub async fn ifenpay_api_post<P, T>(uri: &str, payload: &P, api_key: Option<&str>) -> Result<ApiResponse<T>, ApiErrorResult>
where
    P: Serialize + ?Sized,
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let api_url = env::var("IFENPAY_API_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    let mut request = client.post(&format!("{}{}", api_url, uri)).json(payload);
    if let Some(key) = api_key {
        request = request.header("X-API-Key", key);
    }

    let resp = request.send().await.map_err(|_| ApiErrorType::RequestError.to_response())?;

    parse_response(resp).await
}

pub async fn ifenpay_api_get<T>(uri: &str, api_key: Option<&str>) -> Result<ApiResponse<T>, ApiErrorResult>
where
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let api_url = env::var("IFENPAY_API_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    let mut request = client.get(&format!("{}{}", api_url, uri));
    if let Some(key) = api_key {
        request = request.header("X-API-Key", key);
    }

    let resp = request.send().await.map_err(|_| ApiErrorType::RequestError.to_response())?;

    parse_response(resp).await
}

async fn parse_response<T>(resp: reqwest::Response) -> Result<ApiResponse<T>, ApiErrorResult>
where
    T: DeserializeOwned,
{
    let status = resp.status();
    if !status.is_success() {
        let response = resp.json::<ApiResponse<ApiError>>().await.map_err(|_| ApiErrorType::ParseError.to_response())?;
        return Err((status, Json(response)));
    }

    let response = resp.json::<ApiResponse<T>>().await.map_err(|_| ApiErrorType::ParseError.to_response())?;

    Ok(response)
}
