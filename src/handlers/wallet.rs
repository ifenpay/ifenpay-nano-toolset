use std::thread::sleep;

use axum::{Json};

use crate::{enums::api::error::{account::AccountError, api::ApiErrorType}, helpers::{api::api_success, nano::{nano_address_to_public_key, nano_to_raw}}, 
services::{ifenpay::api::{ifenpay_api_get, ifenpay_api_post}, nano::block::prepare_generate_and_sign_block, wallet::wallet::get_wallet_data}, structs::{api::{ApiErrorResult, ApiSuccessResult}, 
block::{BlockInfoResponseApi, PublishBlockRequestApi, PublishBlockResponseApi}, wallet::{BalanceResponseApi, BlocksPerAccountResponse, SendNanoRequestApi, SendNanoResponseApi, 
    WalletData}}};


pub async fn create_wallet_api() -> Result<ApiSuccessResult<WalletData>, ApiErrorResult> {
    let response = ifenpay_api_get::<WalletData>("/wallet/create", None).await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;
    
    Ok(api_success(response))
}

pub async fn get_balance_api() -> Result<ApiSuccessResult<BalanceResponseApi>, ApiErrorResult> {
    let wallet_data = get_wallet_data();
    let pending_response = ifenpay_api_get::<BlocksPerAccountResponse>(&format!("/wallet/pending/{}", wallet_data.address), Some(&wallet_data.api_key)).await;

    if pending_response.is_ok(){
        let data = pending_response.unwrap().data.unwrap();

        let mut pending_blocks = Vec::new();

        if let Some(blocks) = data.blocks.get(&wallet_data.address) {
            for pending_block in blocks {
                pending_blocks.push(pending_block.clone());
            }
        }
        if pending_blocks.len() > 0 {
            receive_pending_blocks_api(&wallet_data, pending_blocks).await;
        }

        sleep(std::time::Duration::from_secs(1));
    }

	let response = ifenpay_api_get::<BalanceResponseApi>(&format!("/wallet/balance/{}", wallet_data.address), Some(&wallet_data.api_key))
        .await?.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    Ok(api_success(response))
}

async fn receive_pending_blocks_api(wallet_data: &WalletData, pending_blocks: Vec<String>) {
    let api_key = wallet_data.api_key.clone();

    for pending_block in pending_blocks {
        let block_info_response = ifenpay_api_get::<BlockInfoResponseApi>(&format!("/block/info/{}", pending_block), Some(&api_key)).await;
        if let Ok(block_info_response) = block_info_response {
            if let Some(block_info) = block_info_response.data {
                let pending_ammount_u: u128 = block_info.amount.parse().unwrap_or(0); 
                let signed_block = prepare_generate_and_sign_block(wallet_data, pending_ammount_u, &pending_block, true).await;
                if let Ok(signed_block) = signed_block {
                    let request_payload = PublishBlockRequestApi {
                        subtype: signed_block.subtype.clone(),
                        block: signed_block.block.clone(),
                    };
                    let _ = ifenpay_api_post::<PublishBlockRequestApi, PublishBlockResponseApi>("/block/publish", &request_payload, Some(&api_key)).await;
                }
            }
        }
    }
}

pub async fn send_nano_api(Json(payload): Json<SendNanoRequestApi>) -> Result<ApiSuccessResult<SendNanoResponseApi>, ApiErrorResult> {
    let wallet_data = get_wallet_data();    
    let api_key = wallet_data.api_key.clone();
    let amount_raw = nano_to_raw(&payload.amount).unwrap();
    let reciepient_public_key = nano_address_to_public_key(&payload.recipient_address, true)
        .map_err(|_| AccountError::InvalidAddress.to_response())?;

    let signed_block = prepare_generate_and_sign_block(&wallet_data, amount_raw, &reciepient_public_key, false).await?;
    let request_payload = PublishBlockRequestApi {
        subtype: "send".to_string(),
        block: signed_block.block.clone(),
    };
    ifenpay_api_post::<PublishBlockRequestApi, PublishBlockResponseApi>("/block/publish", &request_payload, Some(&api_key)).await?;

    Ok(api_success(SendNanoResponseApi {
        amount: payload.amount,
        recipient: payload.recipient_address,
    }))
}