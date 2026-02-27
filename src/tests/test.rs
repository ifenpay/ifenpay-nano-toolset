#![cfg(test)]

use axum::Json;
use tokio::time::sleep;
use crate::handlers::donate::{donate_api};
use crate::handlers::payment::create_payment_request_api;
use crate::structs::api::{ApiErrorResult, ApiSuccessResult};
use crate::structs::credit::{GetCreditsResponse, TopupCreditsSuccessResponse};
use crate::structs::donate::DonateResponseApi;
use crate::structs::payment::CreatePaymentRequestApi;
use crate::services::wallet::wallet::get_wallet_data;
use crate::structs::wallet::{BalanceResponseApi, SendNanoRequestApi, SendNanoResponseApi};
use axum::extract::Path;
use crate::handlers::payment::get_payment_status_api;
use crate::handlers::credits::{get_credits_api, topup_credits_api};
use crate::handlers::wallet::{get_balance_api, send_nano_api};
use crate::enums::api::error::api::ApiErrorType;


#[cfg(test)]
#[tokio::test]
async fn test_api_calls() -> Result<(), ApiErrorResult> {
    dotenvy::dotenv().ok();

    get_balance().await?;
    // return Ok(());

    send_nano(None, None).await?;
    donate().await?;
    get_credits().await?;
    process_payment().await?;
    // return Ok(());

    // topup_credits().await?;
    Ok(())
}

#[cfg(test)]
async fn get_balance() -> Result<BalanceResponseApi, ApiErrorResult> {
    let get_balance: Result<ApiSuccessResult<BalanceResponseApi>, ApiErrorResult> = get_balance_api().await;
    let get_balance = get_balance?;
    let balance_data = get_balance.0.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    println!("GET_BALANCE RESULT: true {:?}", balance_data);
    Ok(balance_data)
}

#[cfg(test)]
async fn send_nano(address: Option<String>, amount: Option<String>) -> Result<SendNanoResponseApi, ApiErrorResult> {
    let balance = get_balance().await?;

    let send_nano_payload = Json(SendNanoRequestApi {
        recipient_address: address.unwrap_or_else(|| balance.account.clone()), // Send to self for testing if no address is provided
        amount: amount.unwrap_or_else(|| "0.000000001".to_string()), // Send the entire balance for testing if no amount is provided
    });

    let send_result: Result<ApiSuccessResult<SendNanoResponseApi>, ApiErrorResult> = send_nano_api(send_nano_payload).await;
    let send_result = send_result?;
    let send_data = send_result.0.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    println!("SEND_NANO RESULT: true {:?}", send_data);

    sleep(std::time::Duration::from_secs(3)).await;

    get_balance().await?;

    Ok(send_data)
}

#[cfg(test)]
async fn donate() -> Result<DonateResponseApi, ApiErrorResult> {
    let donate_result = donate_api(Path("0.00001".to_string())).await;
    let donate_result = donate_result?;
    let donate_data = donate_result.0.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    println!("DONATE RESULT: true {:?}", donate_data);

    Ok(donate_data)
}

#[cfg(test)]
async fn get_credits() -> Result<GetCreditsResponse, ApiErrorResult> {
    let get_credits = get_credits_api().await;
    let get_credits = get_credits?;
    let credits_data = get_credits.0.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    println!("GET_CREDITS RESULT: true {:?}", credits_data);

    Ok(credits_data)
}

#[cfg(test)]
#[allow(dead_code)]
async fn topup_credits() -> Result<TopupCreditsSuccessResponse, ApiErrorResult> {
    let topup_result = topup_credits_api(Path(10)).await;
    let topup_result = topup_result?;
    let topup_data = topup_result.0.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    println!("TOPUP_CREDITS RESULT: true {:?}", topup_data);

    Ok(topup_data)
}

#[cfg(test)]
async fn process_payment() -> Result<(), ApiErrorResult> {
    let wallet_data = get_wallet_data();

    let create_payment_payload = CreatePaymentRequestApi {
        receive_address: wallet_data.address.clone(),
        amount: "0.00001".to_string(),
        redirect_url: Some("https://example.com/redirect".to_string()),
    };

    let create_payment = create_payment_request_api(Json(create_payment_payload)).await;
    let create_payment = create_payment?;
    let create_payment_data = create_payment.0.data.ok_or_else(|| ApiErrorType::ParseError.to_response())?;

    println!("CREATE_PAYMENT RESULT: true {:?}", create_payment_data);

    let send_nano_payload = Json(SendNanoRequestApi {
        recipient_address: create_payment_data.receive_address.clone(), // Send to self for testing
        amount: create_payment_data.amount.clone(), // Send the entire balance for testing
    });
    println!("payload: {:?}", send_nano_payload);

    let send_nano = send_nano(Some(create_payment_data.receive_address.clone()), Some(create_payment_data.amount.clone())).await;

    println!("SEND_NANO FOR PAYMENT RESULT: {:?}", send_nano.is_ok());
    
    sleep(std::time::Duration::from_secs(5)).await;

    let payment_status = get_payment_status_api(Path(create_payment_data.transaction_id.clone())).await;
    println!("GET_PAYMENT_STATUS RESULT: {:?}", payment_status.is_ok());

    Ok(())
}

#[cfg(test)]
#[allow(dead_code)]
async fn send_remaining_balance() -> Result<(), ApiErrorResult> {
    let balance = get_balance().await?;

    let _ = send_nano(Some("nano_3sttdhxj1ox97z3jjoxx5i4id7m7kk619d9n3u85xmxi5eokygyftp3u61je".to_string()), Some(balance.balance.clone())).await;

    Ok(())
}




