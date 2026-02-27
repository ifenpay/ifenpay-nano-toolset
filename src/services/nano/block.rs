use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::{enums::api::error::{block::BlockError, transaction::TransactionError, work_server::WorkServerError}, helpers::nano::{hex_to_bytes, nano_account_public_key, nano_address_to_public_key, public_key_to_nano_address, sign_hash_with_seed, state_block_hash, 
    work_value}, services::ifenpay::api::ifenpay_api_get, structs::{api::ApiErrorResult, block::{GenerateWorkResponseApi, SignedBlock, StateBlock}, wallet::{AccountInfoResponseApi, WalletData}}};

static LAST_PUBLISHED_FRONTIER: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

const BASE_THRESH_SEND_CHANGE: u64 = 0xfffffff800000000;
const BASE_THRESH_RECEIVE_OPEN_EPOCH: u64 = 0xfffffe0000000000;

const ZERO_FRONTIER: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const DEFAULT_OPEN_REPRESENTATIVE: &str = "nano_37imps4zk1dfahkqweqa91xpysacb7scqxf3jqhktepeofcxqnpx531b3mnt";


pub async fn prepare_generate_and_sign_block(
    wallet_data: &WalletData,
    amount_raw: u128,
    link: &str,
    is_receive: bool,
) -> Result<SignedBlock, ApiErrorResult> {
    let api_key = wallet_data.api_key.clone();

    let mut frontier = ZERO_FRONTIER.to_string();
    let mut balance = "0".to_string();
    let mut representative = wallet_data.public_key.clone();

    if let Ok(account_info) = wait_for_latest_account_info(&wallet_data.address, &api_key).await {
        frontier = account_info.frontier;
        balance = account_info.balance;
        representative = account_info.representative;
    }

    let rep_public_key = resolve_representative_public_key(&frontier, &representative, &wallet_data.public_key);

    let current_balance_u = balance.parse::<u128>().unwrap_or(0);
    if !is_receive && current_balance_u < amount_raw {
        return Err(TransactionError::InsufficientFunds.to_response());
    }

    let subtype = if is_receive {
        if frontier == ZERO_FRONTIER { "open" } else { "receive" }
    } else {
        "send"
    };

    let threshold = if is_receive {
        if frontier == ZERO_FRONTIER {
            BASE_THRESH_RECEIVE_OPEN_EPOCH
        } else {
            BASE_THRESH_RECEIVE_OPEN_EPOCH 
        }
    } else {
        BASE_THRESH_SEND_CHANGE
    };

    let seed_hex = &wallet_data.wallet_private_seed.clone();
    let new_balance_u = if is_receive {
        current_balance_u + amount_raw
    } else {
        current_balance_u - amount_raw
    };
    let work_root = if subtype == "open" {
        wallet_data.public_key.clone()
    } else {
        frontier.clone()
    };


    let account_pub32 = nano_account_public_key(seed_hex, 0)
        .map_err(|_| BlockError::KeyDerivationFailed.to_response())?;
    let prev_32  = hex_to_bytes(&frontier)
        .map_err(|_| BlockError::InvalidPreviousHash.to_response())?;
    let rep_pub32  = hex_to_bytes(&rep_public_key)
        .map_err(|_| BlockError::InvalidRepresentativePublicKey.to_response())?;
    let link_32 = hex_to_bytes(link)
        .map_err(|_| BlockError::InvalidLink.to_response())?;
    let h32 = state_block_hash(&account_pub32, &prev_32, &rep_pub32, new_balance_u, &link_32)
        .map_err(|_| BlockError::BlockHashGenerationFailed.to_response())?;
    let sig_hex = sign_hash_with_seed(&seed_hex, 0, &h32)
        .map_err(|_| BlockError::SigningFailed.to_response())?;
    let work_hex = generate_work(&work_root, &threshold).await?;

    let block = StateBlock {
        block_type: "state".to_string(),
        account: wallet_data.address.clone(),
        previous: frontier.clone(),
        representative: public_key_to_nano_address(&rep_public_key)
            .map_err(|_| BlockError::InvalidRepresentativeAddress.to_response())?,
        balance: new_balance_u.to_string(),
        link: link.to_string(),
        signature: sig_hex,
        work: work_hex.work.clone(),
    };
    
    let mut work_le = hex_to_bytes(&work_hex.work)
        .map_err(|_| BlockError::InvalidWorkHex.to_response())?;
    work_le.reverse();
    let work_le: [u8; 8] = work_le.try_into()
        .map_err(|_| BlockError::InvalidWorkLength.to_response())?;

    let work_root_bytes: [u8; 32] = hex_to_bytes(&work_root)
        .map_err(|_| BlockError::InvalidWorkRoot.to_response())?
        .try_into()
        .map_err(|_| BlockError::InvalidWorkRootLength.to_response())?;

    let work_val = work_value(&work_root_bytes, &work_le)
        .map_err(|_| BlockError::CalculateWorkFailed.to_response())?;

    Ok(SignedBlock {
        block,
        hash: hex::encode(h32),
        account_public_key: wallet_data.public_key.clone(),
        subtype: subtype.to_string(),
        work_root: work_root.clone(),
        threshold_hex: format!("0x{:016x}", threshold),
        work_value_hex: format!("0x{:016x}", work_val),
    })

}

async fn generate_work(
    hash: &str, 
    threshold: &u64
) -> Result<GenerateWorkResponseApi, ApiErrorResult> {
    let client = reqwest::Client::new();
    let threshold_hex = format!("{:016x}", threshold);
    let work_server_url = std::env::var("NANO_WORK_SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:4000".to_string());
    let response_text = client.post(&work_server_url)
    .json(&serde_json::json!({
        "action": "work_generate",
        "hash": hash,
        "difficulty": threshold_hex
    }))
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    if let Ok(work_response) = serde_json::from_str::<GenerateWorkResponseApi>(&response_text) {
        return Ok(work_response);
    } else {
        return Err(WorkServerError::Error(response_text).to_response());
    }
}

async fn wait_for_latest_account_info(address: &str, api_key: &str) -> Result<AccountInfoResponseApi, ()> {
    let mut account_info_response = ifenpay_api_get::<AccountInfoResponseApi>(&format!("/account/info/{}", address), Some(api_key)).await;
    let mut loop_attempts = 0;

    loop {
        if let Ok(account_info) = &account_info_response {
            let account_info = match &account_info.data {
                Some(data) => data,
                None => {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    account_info_response = ifenpay_api_get::<AccountInfoResponseApi>(&format!("/account/info/{}", address), Some(api_key)).await;
                    loop_attempts += 1;
                    if loop_attempts >= 10 {
                        break;
                    }
                    continue;
                }
            };

            if LAST_PUBLISHED_FRONTIER.lock().unwrap().as_deref() != Some(&account_info.frontier) {
                *LAST_PUBLISHED_FRONTIER.lock().unwrap() = Some(account_info.frontier.clone());
                return Ok(account_info.clone());
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        account_info_response = ifenpay_api_get::<AccountInfoResponseApi>(&format!("/account/info/{}", address), Some(api_key)).await;

        loop_attempts += 1;
        if loop_attempts >= 10 {
            break;
        }
    }


    if account_info_response.is_err() {
        return Err(());
    }

    account_info_response.unwrap().data.ok_or(()).map_err(|_| ())
}

fn resolve_representative_public_key(frontier: &str, representative: &str, public_key: &str) -> String {
    if frontier == ZERO_FRONTIER {
        return nano_address_to_public_key(DEFAULT_OPEN_REPRESENTATIVE, true)
            .unwrap_or_else(|_| public_key.to_lowercase());
    }

    if !representative.is_empty() {
        if representative.starts_with("nano_") || representative.starts_with("xrb_") {
            nano_address_to_public_key(representative, true).unwrap_or_else(|_| public_key.to_lowercase())
        } else if representative.len() == 64 {
            representative.to_lowercase()
        } else {
            public_key.to_lowercase()
        }
    } else {
        public_key.to_lowercase()
    }
}