use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
use hex::FromHex;

use crate::enums::api::error::account::AccountError;
use crate::enums::api::error::transaction::TransactionError;
use crate::structs::api::ApiErrorResult;

fn blake2b512(data: &[u8]) -> Result<[u8; 64], String> {
    let mut hasher = Blake2bVar::new(64).map_err(|e| format!("Invalid blake2b size: {}", e))?;
    hasher.update(data);
    let mut out = [0u8; 64];
    hasher
        .finalize_variable(&mut out)
        .map_err(|e| format!("Failed to finalize blake2b: {}", e))?;
    Ok(out)
}

fn clamp_scalar_bytes(mut scalar: [u8; 32]) -> [u8; 32] {
    scalar[0] &= 248;
    scalar[31] &= 127;
    scalar[31] |= 64;
    scalar
}


pub fn nano_to_raw(nano_str: &str) -> Result<u128, ApiErrorResult> {
    let nano = nano_str.trim();
    
    if nano.is_empty() {
        return Err(TransactionError::InvalidNumberFormat.to_response());
    }
    
    if nano.starts_with('-') {
        return Err(TransactionError::InvalidNegativeAmount.to_response());
    }
    
    let parts: Vec<&str> = nano.split('.').collect();
    if parts.len() > 2 {
        return Err(TransactionError::InvalidNumberFormat.to_response());
    }
    
    let whole_str = parts[0];
    let frac_str = if parts.len() == 2 { parts[1] } else { "" };
    
    if !whole_str.is_empty() && !whole_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(TransactionError::InvalidWholeNumber.to_response());
    }
    
    if !frac_str.is_empty() && !frac_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(TransactionError::InvalidFractionalPart.to_response());
    }
    
    if frac_str.len() > 30 {
        return Err(TransactionError::TooManyDecimalPlaces.to_response());
    }
    
    let frac_padded = format!("{:0<30}", frac_str);
    
    let whole: u128 = if whole_str.is_empty() || whole_str == "0" {
        0
    } else {
        whole_str.parse().map_err(|_| TransactionError::InvalidWholeNumber.to_response())?
    };
    
    let fractional: u128 = if frac_padded.is_empty() {
        0
    } else {
        frac_padded.parse().map_err(|_| TransactionError::InvalidFractionalPart.to_response())?
    };
    
    let raw = whole.checked_mul(1_000_000_000_000_000_000_000_000_000_000)
        .ok_or(TransactionError::AmountToLarge.to_response())?
        .checked_add(fractional)
        .ok_or(TransactionError::AmountToLarge.to_response())?;
    
    Ok(raw)
}

pub fn hex_to_bytes(h: &str) -> Result<Vec<u8>, String> {
    let h = h.trim().to_lowercase().replace("0x", "");
    Vec::from_hex(&h).map_err(|e| format!("Invalid hex: {}", e))
}

pub fn u64_le(b: &[u8]) -> Result<u64, String> {
    if b.len() < 8 {
        return Err("Need at least 8 bytes".to_string());
    }
    let arr: [u8; 8] = b[..8].try_into().unwrap();
    Ok(u64::from_le_bytes(arr))
}

pub fn blake2b8(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2bVar::new(8).expect("invalid blake2b output size");
    hasher.update(data);
    let mut out = [0u8; 8];
    hasher
        .finalize_variable(&mut out)
        .expect("failed to finalize blake2b8");
    out.to_vec()
}

pub fn work_value(root32: &[u8], work_le8: &[u8]) -> Result<u64, String> {
    let mut concat = Vec::from(work_le8);
    concat.extend_from_slice(root32);
    let h = blake2b8(&concat);
    u64_le(&h)
}

pub fn nano_account_private_key(seed_hex: &str, index: u32) -> Result<Vec<u8>, String> {
    let seed = hex_to_bytes(seed_hex)?;
    let i = index.to_be_bytes();
    let mut data = seed;
    data.extend_from_slice(&i);
    
    let mut hasher = Blake2bVar::new(32).map_err(|e| format!("Invalid blake2b size: {}", e))?;
    hasher.update(&data);
    let mut out = [0u8; 32];
    hasher
        .finalize_variable(&mut out)
        .map_err(|e| format!("Failed to finalize blake2b: {}", e))?;
    Ok(out.to_vec())
}

pub fn nano_account_public_key(seed_hex: &str, index: u32) -> Result<Vec<u8>, String> {
    let priv32 = nano_account_private_key(seed_hex, index)?;
    if priv32.len() != 32 {
        return Err("Private key must be 32 bytes".to_string());
    }

    let mut priv_arr = [0u8; 32];
    priv_arr.copy_from_slice(&priv32);
    let h = blake2b512(&priv_arr)?;

    let mut scalar_bytes = [0u8; 32];
    scalar_bytes.copy_from_slice(&h[..32]);
    let scalar = Scalar::from_bytes_mod_order(clamp_scalar_bytes(scalar_bytes));

    let public_point = scalar * ED25519_BASEPOINT_POINT;
    Ok(public_point.compress().to_bytes().to_vec())
}

pub fn state_block_hash(
    account_pub32: &Vec<u8>,
    previous32: &Vec<u8>,
    representative_pub32: &Vec<u8>,
    balance_u128: u128,
    link32: &Vec<u8>,
) -> Result<Vec<u8>, String> {
    if account_pub32.len() != 32 {
        return Err("account_pub32 must be 32 bytes".to_string());
    }
    if previous32.len() != 32 {
        return Err("previous32 must be 32 bytes".to_string());
    }
    if representative_pub32.len() != 32 {
        return Err("representative_pub32 must be 32 bytes".to_string());
    }
    if link32.len() != 32 {
        return Err("link32 must be 32 bytes".to_string());
    }
    
    let mut preamble = vec![0u8; 31];
    preamble.push(0x06);
    
    let bal16 = balance_u128.to_be_bytes();
    
    let mut hasher = Blake2bVar::new(32).map_err(|e| format!("Invalid blake2b size: {}", e))?;
    hasher.update(&preamble);
    hasher.update(account_pub32);
    hasher.update(previous32);
    hasher.update(representative_pub32);
    hasher.update(&bal16);
    hasher.update(link32);
    let mut out = [0u8; 32];
    hasher
        .finalize_variable(&mut out)
        .map_err(|e| format!("Failed to finalize blake2b: {}", e))?;
    Ok(out.to_vec())
}

pub fn sign_hash_with_seed(seed_hex: &str, index: u32, h32: &[u8]) -> Result<String, String> {
    let priv32 = nano_account_private_key(seed_hex, index)?;
    if priv32.len() != 32 {
        return Err("Private key must be 32 bytes".to_string());
    }

    let mut priv_arr = [0u8; 32];
    priv_arr.copy_from_slice(&priv32);
    let h = blake2b512(&priv_arr)?;

    let mut a_bytes = [0u8; 32];
    a_bytes.copy_from_slice(&h[..32]);
    let a = Scalar::from_bytes_mod_order(clamp_scalar_bytes(a_bytes));

    let public_key = nano_account_public_key(seed_hex, index)?;

    let mut nonce_input = Vec::with_capacity(32 + h32.len());
    nonce_input.extend_from_slice(&h[32..64]);
    nonce_input.extend_from_slice(h32);
    let r_hash = blake2b512(&nonce_input)?;
    let r = Scalar::from_bytes_mod_order_wide(&r_hash);

    let r_point = r * ED25519_BASEPOINT_POINT;
    let r_bytes = r_point.compress().to_bytes();

    let mut k_input = Vec::with_capacity(32 + public_key.len() + h32.len());
    k_input.extend_from_slice(&r_bytes);
    k_input.extend_from_slice(&public_key);
    k_input.extend_from_slice(h32);
    let k_hash = blake2b512(&k_input)?;
    let k = Scalar::from_bytes_mod_order_wide(&k_hash);

    let s = r + (k * a);

    let mut sig = [0u8; 64];
    sig[..32].copy_from_slice(&r_bytes);
    sig[32..].copy_from_slice(&s.to_bytes());

    Ok(hex::encode(sig))
}

const NANO_ALPH: &[u8; 32] = b"13456789abcdefghijkmnopqrstuwxyz";

pub fn nano_base32_encode(data: &[u8]) -> String {
    let mut value: u32 = 0;
    let mut bits: u8 = 0;
    let mut out = Vec::new();

    for &byte in data {
        value = (value << 8) | byte as u32;
        bits += 8;

        while bits >= 5 {
            let idx = ((value >> (bits - 5)) & 31) as usize;
            out.push(NANO_ALPH[idx]);
            bits -= 5;
        }
    }

    if bits > 0 {
        let idx = ((value << (5 - bits)) & 31) as usize;
        out.push(NANO_ALPH[idx]);
    }

    String::from_utf8(out).expect("valid nano base32 output")
}

pub fn nano_base32_decode_bytes(s: &str) -> Result<Vec<u8>, String> {
    let mut alph_map = [0u8; 256];
    for (i, &c) in NANO_ALPH.iter().enumerate() {
        alph_map[c as usize] = i as u8;
    }

    let mut out = Vec::new();
    let mut value: u32 = 0;
    let mut bits: u8 = 0;

    for ch in s.bytes() {
        if (ch as usize) >= alph_map.len() {
            return Err(format!("Invalid base32 char: {}", ch as char));
        }
        let idx = alph_map[ch as usize];
        if idx == 0 && ch != b'1' {
            return Err(format!("Invalid base32 char: {}", ch as char));
        }

        value = (value << 5) | idx as u32;
        bits += 5;

        while bits >= 8 {
            let byte = ((value >> (bits - 8)) & 0xff) as u8;
            out.push(byte);
            bits -= 8;
        }
    }

    if bits > 0 {
        let byte = ((value << (8 - bits)) & 0xff) as u8;
        out.push(byte);
    }

    Ok(out)
}

pub fn nano_base32_decode_to_bytes_32(s: &str) -> Result<[u8; 32], String> {
    if s.len() != 52 {
        return Err(format!("Expected 52 chars, got {}", s.len()));
    }

    let full = format!("1111{}", s);
    let decoded = nano_base32_decode_bytes(&full)?;
    if decoded.len() < 35 {
        return Err(format!("Decoded {} bytes, expected at least 35", decoded.len()));
    }

    let mut result = [0u8; 32];
    result.copy_from_slice(&decoded[decoded.len() - 32..]);

    Ok(result)
}

pub fn public_key_to_nano_address(public_key_hex: &str) -> Result<String, String> {
    let pub_bytes = hex_to_bytes(public_key_hex)?;
    if pub_bytes.len() != 32 {
        return Err("public_key must be 32 bytes hex".to_string());
    }
    
    let mut hasher = Blake2bVar::new(5).map_err(|e| format!("Invalid blake2b size: {}", e))?;
    hasher.update(&pub_bytes);
    let mut chk = [0u8; 5];
    hasher
        .finalize_variable(&mut chk)
        .map_err(|e| format!("Failed to finalize blake2b: {}", e))?;
    let mut chk = chk.to_vec();
    chk.reverse();
    
    let mut key_input = vec![0u8, 0u8, 0u8];
    key_input.extend_from_slice(&pub_bytes);
    let key_part = nano_base32_encode(&key_input)[4..].to_string();
    let chk_part = nano_base32_encode(&chk);
    
    Ok(format!("nano_{}{}", key_part, chk_part))
}

pub fn nano_address_to_public_key(address: &str, verify_checksum: bool) -> Result<String, ApiErrorResult> {
    let address = address.trim();
    let body = if address.starts_with("nano_") {
        &address[5..]
    } else if address.starts_with("xrb_") {
        &address[4..]
    } else {
        return Err(AccountError::InvalidAddress.to_response());
    };
    
    if body.len() != 60 {
        return Err(AccountError::InvalidAddress.to_response());
    }
    
    let key_part = &body[..52];
    let chk_part = &body[52..];
    
    let pubkey = nano_base32_decode_to_bytes_32(key_part).map_err(|_| AccountError::InvalidAddress.to_response())?;
    
    if verify_checksum {
        let mut hasher = Blake2bVar::new(5).map_err(|_| AccountError::InvalidAddress.to_response())?;
        hasher.update(&pubkey);
        let mut chk_calc = [0u8; 5];
        hasher
            .finalize_variable(&mut chk_calc)
            .map_err(|_| AccountError::InvalidAddress.to_response())?;
        let mut chk_calc = chk_calc.to_vec();
        chk_calc.reverse();
        
        let chk_given = nano_base32_decode_bytes(chk_part).map_err(|_| AccountError::InvalidAddress.to_response())?;
        let chk_given = &chk_given[chk_given.len().saturating_sub(5)..];

        if chk_given != chk_calc.as_slice() {
            return Err(AccountError::InvalidAddress.to_response());
        }
    }
    
    Ok(hex::encode(pubkey))
}
