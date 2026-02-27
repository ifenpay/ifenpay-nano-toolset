use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use argon2::Argon2;
use serde_json;
use crate::structs::wallet::WalletData;

pub fn get_wallet_data() -> WalletData {
    let file_path = "data/ai.bin";
    let encrypted_data = std::fs::read(file_path).expect("Failed to read wallet file");
    let wallet_password = std::env::var("AI_WALLET_PASSWORD").expect("AI_WALLET_PASSWORD not set in .env");

    decrypt_wallet_data(&encrypted_data, &wallet_password).expect("Failed to decrypt wallet data")
}

pub fn encrypt_wallet_data(
    wallet_data: &WalletData,
    password: &str,
) -> Result<Vec<u8>, String> {

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|e| e.to_string())?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = serde_json::to_vec(wallet_data).map_err(|e| e.to_string())?;

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .map_err(|_| "Encryption failed".to_string())?;

    let mut output = Vec::with_capacity(16 + 12 + ciphertext.len());
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}


pub fn decrypt_wallet_data(
    data: &[u8],
    password: &str,
) -> Result<WalletData, String> {

    if data.len() < 28 {
        return Err("Data too short".into());
    }

    let salt = &data[..16];
    let nonce = &data[16..28];
    let ciphertext = &data[28..];

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Wrong password or corrupted data".to_string())?;

    let wallet: WalletData =
        serde_json::from_slice(&plaintext).map_err(|e| e.to_string())?;

    Ok(wallet)
}