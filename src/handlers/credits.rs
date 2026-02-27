use axum::{extract::Path};

use crate::{enums::api::error::{api::ApiErrorType, credit::CreditError}, helpers::{api::api_success, nano::{nano_address_to_public_key, nano_to_raw}}, services::{ifenpay::api::{ifenpay_api_get, ifenpay_api_post}, nano::block::prepare_generate_and_sign_block,
    wallet::wallet::get_wallet_data}, structs::{api::{ApiErrorResult, ApiSuccessResult}, credit::{GetCreditsResponse, TopupCreditsFinalRequest, TopupCreditsResponse, TopupCreditsSuccessResponse}}};

pub async fn get_credits_api() -> Result<ApiSuccessResult<GetCreditsResponse>, ApiErrorResult> {
    let wallet_data = get_wallet_data();
    let response = ifenpay_api_get::<GetCreditsResponse>("/credits", Some(&wallet_data.api_key)).await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    Ok(api_success(response))
}

pub async fn topup_credits_api(Path(credits_amount): Path<u32>) -> Result<ApiSuccessResult<TopupCreditsSuccessResponse>, ApiErrorResult> {
    is_valid_topup_amount(&credits_amount.to_string())?;

    let active_wallet = get_wallet_data();
    let create_payment_response = ifenpay_api_get::<TopupCreditsResponse>(&format!("/credits/topup/{}", credits_amount), Some(&active_wallet.api_key)).await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;
    
    let amount_raw = nano_to_raw(&create_payment_response.nano_amount).unwrap_or(0);
    let recipient_public_key = nano_address_to_public_key(&create_payment_response.payment_address,true).unwrap();
    
    let block = prepare_generate_and_sign_block(&active_wallet, amount_raw, &recipient_public_key, false).await?;
    let topup_payload = TopupCreditsFinalRequest {
        subtype: block.subtype.clone(),
        block: block.block.clone(),
        transaction_id: create_payment_response.transaction_id.clone(),
    };

    let topup_result = ifenpay_api_post::<TopupCreditsFinalRequest, TopupCreditsSuccessResponse>("/credits/topup", &topup_payload, Some(&active_wallet.api_key)).await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;
    
    Ok(api_success(topup_result))
}

pub fn is_valid_topup_amount(amount: &str) -> Result<(), ApiErrorResult> {
    const VALID_OPTIONS: [u32; 9] = [10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000];

    match amount.parse::<u32>() {
        Ok(amt) if VALID_OPTIONS.contains(&amt) => Ok(()),
        _ => Err(CreditError::InvalidCreditsAmount.to_response()),
    }
}