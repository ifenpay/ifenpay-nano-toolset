use axum::Json;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::enums::api::error::api::ApiErrorType;
use crate::managers::env_manager::IFENPAY_API_URL;
use crate::structs::api::{ApiError, ApiErrorResult, ApiResponse};

pub async fn ifenpay_api_post<P, T>(uri: &str, payload: &P, api_key: Option<&str>) -> Result<ApiResponse<T>, ApiErrorResult>
where
    P: Serialize + ?Sized,
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let mut request = client.post(&format!("{}{}", *IFENPAY_API_URL, uri)).json(payload);
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
    let mut request = client.get(&format!("{}{}", *IFENPAY_API_URL, uri));
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
