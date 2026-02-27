use axum::{Json, extract::Path};

use crate::{enums::api::error::api::ApiErrorType, handlers::wallet::send_nano_api, helpers::api::api_success, services::{ifenpay::api::ifenpay_api_get, wallet::wallet::get_wallet_data}, structs::{api::{ApiErrorResult, ApiSuccessResult}, donate::{DonateAddressResponse, DonateResponseApi}, wallet::SendNanoRequestApi}};

pub async fn donate_api(
    Path(amount): Path<String>
) -> Result<ApiSuccessResult<DonateResponseApi>, ApiErrorResult> {
    let wallet_data = get_wallet_data();

    let donate_address = ifenpay_api_get::<DonateAddressResponse>("/donate/address", Some(&wallet_data.api_key)).await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    let result = send_nano_api(Json(SendNanoRequestApi {
        recipient_address: donate_address.address.clone(),
        amount: amount.clone(),
    })).await;

    Ok(api_success(DonateResponseApi {
        success: result.is_ok(),
        amount,
        message: "Thank you for your donation!".to_string(),
    }))
}

