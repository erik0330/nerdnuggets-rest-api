use crate::state::AppState;
use axum::{extract::State, Json};
use chrono::Utc;
use ethers::core::types::{Address, Signature};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use types::{
    dto::{ClaimRequest, ClaimResponse, TelegramLoginRequest, VerifyTelegramResponse},
    error::{ApiError, DbError, UserError},
};

type HmacSha256 = Hmac<Sha256>;

/// Verify Telegram login data and check if user has unclaimed points
pub async fn verify_telegram(
    State(state): State<AppState>,
    Json(payload): Json<TelegramLoginRequest>,
) -> Result<Json<VerifyTelegramResponse>, ApiError> {
    // Validate input lengths to prevent DoS
    if payload.hash.len() > 128 {
        return Ok(Json(VerifyTelegramResponse {
            success: false,
            points: None,
            telegram_id: None,
            username: None,
            error: Some("Invalid Telegram login".to_string()),
        }));
    }

    // Validate Telegram login data
    let bot_token = &state.env.telegram_bot_token;

    // Create data_check_string: all fields except hash, sorted alphabetically, joined as "key=value\n..."
    let mut data_map: BTreeMap<String, String> = BTreeMap::new();
    data_map.insert("auth_date".to_string(), payload.auth_date.to_string());
    data_map.insert("first_name".to_string(), payload.first_name.clone());
    data_map.insert("id".to_string(), payload.id.to_string());

    if let Some(ref last_name) = payload.last_name {
        data_map.insert("last_name".to_string(), last_name.clone());
    }
    if let Some(ref photo_url) = payload.photo_url {
        data_map.insert("photo_url".to_string(), photo_url.clone());
    }
    if let Some(ref username) = payload.username {
        data_map.insert("username".to_string(), username.clone());
    }

    let data_check_string: String = data_map
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("\n");

    // Compute secret_key = SHA256(bot_token)
    let mut hasher = Sha256::new();
    hasher.update(bot_token.as_bytes());
    let secret_key = hasher.finalize();

    // Compute expected_hash = HMAC-SHA256(data_check_string, secret_key)
    let mut mac = HmacSha256::new_from_slice(&secret_key)
        .map_err(|_| ApiError::DbError(DbError::Str("HMAC initialization failed".to_string())))?;
    mac.update(data_check_string.as_bytes());
    let expected_hash = mac.finalize();
    let expected_hash_hex = hex::encode(expected_hash.into_bytes());

    // Compare to received hash (hex lowercase)
    // Using simple equality comparison - timing attacks not a concern here as hash is user-provided
    if expected_hash_hex != payload.hash.to_lowercase() {
        return Ok(Json(VerifyTelegramResponse {
            success: false,
            points: None,
            telegram_id: None,
            username: None,
            error: Some("Invalid Telegram login".to_string()),
        }));
    }

    // Check auth_date is within last 24 hours
    let auth_timestamp = payload.auth_date;
    let now = Utc::now().timestamp();
    let twenty_four_hours = 24 * 60 * 60;
    if now - auth_timestamp > twenty_four_hours {
        return Ok(Json(VerifyTelegramResponse {
            success: false,
            points: None,
            telegram_id: None,
            username: None,
            error: Some("Invalid Telegram login".to_string()),
        }));
    }

    // Look up user in database by telegram_id
    let telegram_id_str = payload.id.to_string();
    let holder = state
        .service
        .holder
        .get_holder_by_telegram_id(&telegram_id_str)
        .await;

    match holder {
        None => Ok(Json(VerifyTelegramResponse {
            success: false,
            points: None,
            telegram_id: None,
            username: None,
            error: Some("No points found".to_string()),
        })),
        Some(h) => {
            // Check if already claimed
            if h.claimed_at.is_some() {
                return Ok(Json(VerifyTelegramResponse {
                    success: false,
                    points: None,
                    telegram_id: None,
                    username: None,
                    error: Some("Already claimed".to_string()),
                }));
            }

            // Check if has points
            if h.current_coins <= 0 {
                return Ok(Json(VerifyTelegramResponse {
                    success: false,
                    points: None,
                    telegram_id: None,
                    username: None,
                    error: Some("No points found".to_string()),
                }));
            }

            Ok(Json(VerifyTelegramResponse {
                success: true,
                points: Some(h.current_coins as u64),
                telegram_id: Some(payload.id as u64),
                username: h.username.clone(),
                error: None,
            }))
        }
    }
}

/// Claim points by signing a message with a wallet
pub async fn claim(
    State(state): State<AppState>,
    Json(payload): Json<ClaimRequest>,
) -> Result<Json<ClaimResponse>, ApiError> {
    // Validate input length
    if payload.wallet_address.len() > 255 || payload.message.len() > 10000 {
        return Ok(Json(ClaimResponse {
            success: false,
            points_claimed: None,
            wallet: None,
            error: Some("Invalid input".to_string()),
        }));
    }

    // Re-verify telegramId exists and has unclaimed points (prevent tampering)
    let telegram_id_str = payload.telegram_id.to_string();
    let holder = state
        .service
        .holder
        .get_holder_by_telegram_id(&telegram_id_str)
        .await
        .ok_or_else(|| {
            ApiError::DbError(DbError::Str("Holder not found".to_string()))
        })?;

    // Check if already claimed
    if holder.claimed_at.is_some() {
        return Ok(Json(ClaimResponse {
            success: false,
            points_claimed: None,
            wallet: None,
            error: Some("Already claimed".to_string()),
        }));
    }

    // Check if has points
    if holder.current_coins <= 0 {
        return Ok(Json(ClaimResponse {
            success: false,
            points_claimed: None,
            wallet: None,
            error: Some("No points found".to_string()),
        }));
    }

    // Verify wallet address format
    let wallet_address = payload.wallet_address.parse::<Address>().map_err(|_| {
        ApiError::UserError(UserError::Str("Invalid wallet address format".to_string()))
    })?;

    // Recover signer address using ethers
    // Parse signature
    let signature = payload
        .signature
        .parse::<Signature>()
        .map_err(|_| {
            ApiError::UserError(UserError::Str("Invalid signature format".to_string()))
        })?;

    // Recover address from message
    // Ethereum signed message format: "\x19Ethereum Signed Message:\n<len>\n<message>"
    // Where len is the decimal string representation of message length
    let message_bytes = payload.message.as_bytes();
    let message_len_str = message_bytes.len().to_string();
    let mut prefixed_message = Vec::with_capacity(26 + message_len_str.len() + message_bytes.len());
    prefixed_message.extend_from_slice(b"\x19Ethereum Signed Message:\n");
    prefixed_message.extend_from_slice(message_len_str.as_bytes());
    prefixed_message.extend_from_slice(message_bytes);

    // Hash the prefixed message using keccak256
    let message_hash = ethers::core::utils::keccak256(&prefixed_message);

    // Recover the address from the signature
    let recovered_address = signature
        .recover(message_hash)
        .map_err(|_| {
            ApiError::UserError(UserError::Str("Failed to recover address".to_string()))
        })?;

    // Compare addresses (Address implements PartialEq and handles case-insensitive comparison)
    if recovered_address != wallet_address {
        return Ok(Json(ClaimResponse {
            success: false,
            points_claimed: None,
            wallet: None,
            error: Some("Signature does not match wallet address".to_string()),
        }));
    }

    // Verify message contains correct telegramId and/or username
    // Use regex or exact matching to prevent substring attacks
    // For telegram ID: must be exact match as a word boundary
    let telegram_id_str_in_message = payload.telegram_id.to_string();
    let telegram_id_in_message = payload
        .message
        .split_whitespace()
        .any(|word| word == &telegram_id_str_in_message);

    let username_in_message = holder
        .username
        .as_ref()
        .map(|u| {
            // Username must match exactly (with or without @ prefix)
            payload
                .message
                .split_whitespace()
                .any(|word| word == u || word == &format!("@{}", u))
        })
        .unwrap_or(false);

    if !telegram_id_in_message && !username_in_message {
        return Ok(Json(ClaimResponse {
            success: false,
            points_claimed: None,
            wallet: None,
            error: Some("Message must contain telegram ID or username".to_string()),
        }));
    }

    // Mark as claimed in DB (atomic operation prevents race conditions)
    let updated = state
        .service
        .holder
        .update_holder_claim(&telegram_id_str, &payload.wallet_address)
        .await?;

    // Check if the update actually succeeded (prevents race conditions)
    if !updated {
        return Ok(Json(ClaimResponse {
            success: false,
            points_claimed: None,
            wallet: None,
            error: Some("Claim failed - may have already been claimed or points are zero".to_string()),
        }));
    }

    // Return success
    Ok(Json(ClaimResponse {
        success: true,
        points_claimed: Some(holder.current_coins as u64),
        wallet: Some(payload.wallet_address),
        error: None,
    }))
}

