use axum::{Json, extract::Path};
use serde_json::Value;

use crate::{
    enums::api::error::api::ApiErrorType,
    helpers::api::api_success,
    services::{ifenpay::api::{ifenpay_api_get, ifenpay_api_post}, wallet::wallet::get_wallet_data},
    structs::{api::{ApiErrorResult, ApiResponse, ApiSuccessResult}, payment::{CreatePaymentRequestApi, CreatePaymentResponseApi, StatusPaymentnApi}},
};

pub async fn create_payment_request_api(
    Json(payload): Json<CreatePaymentRequestApi>
) -> Result<ApiSuccessResult<CreatePaymentResponseApi>, ApiErrorResult> {
    let wallet_data = get_wallet_data();
    let response = ifenpay_api_post("/payment/request", &payload, Some(&wallet_data.api_key)).await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    Ok(api_success(response))
}

pub async fn get_payment_status_api(
    Path(transaction_id): Path<String>
) -> Result<ApiSuccessResult<StatusPaymentnApi>, ApiErrorResult> {
    let wallet_data = get_wallet_data();
    let response = ifenpay_api_get::<Value>(&format!("/payment/status/{}", transaction_id), Some(&wallet_data.api_key)).await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;
    let normalized = normalize_payment_status(response, &transaction_id).ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    Ok(api_success(normalized))
}

fn normalize_payment_status(raw: Value, fallback_transaction_id: &str) -> Option<StatusPaymentnApi> {
    if let Ok(direct) = serde_json::from_value::<StatusPaymentnApi>(raw.clone()) {
        return Some(direct);
    }

    if let Ok(wrapped) = serde_json::from_value::<ApiResponse<StatusPaymentnApi>>(raw.clone()) {
        if let Some(data) = wrapped.data {
            return Some(data);
        }
    }

    let payload = if let Ok(wrapped_raw) = serde_json::from_value::<ApiResponse<Value>>(raw.clone()) {
        wrapped_raw.data.unwrap_or(raw)
    } else {
        raw
    };

    let object = payload.as_object()?;

    let transaction_id = object
        .get("transaction_id")
        .and_then(Value::as_str)
        .or_else(|| object.get("transactionId").and_then(Value::as_str))
        .unwrap_or(fallback_transaction_id)
        .to_string();

    let message = object
        .get("message")
        .and_then(Value::as_str)
        .or_else(|| object.get("status").and_then(Value::as_str))
        .unwrap_or("Payment status retrieved")
        .to_string();

    let is_paid = object
        .get("is_paid")
        .and_then(parse_bool)
        .or_else(|| object.get("isPaid").and_then(parse_bool))
        .or_else(|| object.get("paid").and_then(parse_bool))
        .or_else(|| object.get("status").and_then(parse_status_to_paid))
        .unwrap_or(false);

    let success = object
        .get("success")
        .and_then(parse_bool)
        .unwrap_or(true);

    Some(StatusPaymentnApi {
        success,
        message,
        transaction_id,
        is_paid,
    })
}

fn parse_bool(value: &Value) -> Option<bool> {
    if let Some(boolean) = value.as_bool() {
        return Some(boolean);
    }

    let as_str = value.as_str()?.to_ascii_lowercase();
    match as_str.as_str() {
        "true" | "1" | "yes" | "paid" | "confirmed" | "completed" | "success" => Some(true),
        "false" | "0" | "no" | "pending" | "unpaid" | "open" | "processing" => Some(false),
        _ => None,
    }
}

fn parse_status_to_paid(value: &Value) -> Option<bool> {
    let status = value.as_str()?.to_ascii_lowercase();
    match status.as_str() {
        "paid" | "confirmed" | "completed" | "success" => Some(true),
        "pending" | "unpaid" | "open" | "processing" => Some(false),
        _ => None,
    }
}