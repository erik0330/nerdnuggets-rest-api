use crate::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use std::collections::HashMap;
use types::dto::MerkleProofResponse;
use types::error::{ApiError, DbError, UserError};

/// Get Merkle proof for a user to claim their Telegram points
/// GET /api/v2/merkle-proof?telegramId=X&walletAddress=Y
pub async fn get_merkle_proof(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<MerkleProofResponse>, ApiError> {
    // Get query parameters
    let telegram_id = params.get("telegramId").ok_or_else(|| {
        ApiError::UserError(UserError::Str("telegramId query parameter is required".to_string()))
    })?;

    let wallet_address = params.get("walletAddress").ok_or_else(|| {
        ApiError::UserError(UserError::Str("walletAddress query parameter is required".to_string()))
    })?;

    // Validate wallet address format
    let _wallet = wallet_address.parse::<ethers::types::Address>().map_err(|_| {
        ApiError::UserError(UserError::Str("Invalid wallet address format".to_string()))
    })?;

    // Get holder from database
    let holder = state
        .service
        .holder
        .get_holder_by_telegram_id(telegram_id)
        .await
        .ok_or_else(|| {
            ApiError::DbError(DbError::Str("Holder not found".to_string()))
        })?;

    // Check if already claimed
    if holder.claimed_at.is_some() {
        return Err(ApiError::UserError(UserError::Str("Already claimed".to_string())));
    }

    // Check if has points
    if holder.current_coins <= 0 {
        return Err(ApiError::UserError(UserError::Str("No points found".to_string())));
    }

    // Get Merkle proof
    let proof_bytes = state
        .service
        .merkle
        .get_proof(telegram_id, holder.current_coins as u64)
        .await?;

    // Get Merkle root
    let root_bytes = state
        .service
        .merkle
        .get_root()
        .await
        .ok_or_else(|| {
            ApiError::UserError(UserError::Str("Merkle tree not initialized".to_string()))
        })?;

    // Convert proof bytes to hex strings (with 0x prefix)
    let proof_hex: Vec<String> = proof_bytes.iter().map(|b| format!("0x{}", hex::encode(b))).collect();

    // Convert root to hex string (with 0x prefix)
    let root_hex = format!("0x{}", hex::encode(root_bytes));

    Ok(Json(MerkleProofResponse {
        telegram_id: holder.telegram_id,
        wallet_address: wallet_address.clone(),
        token_amount: holder.current_coins as u64,
        proof: proof_hex,
        merkle_root: root_hex,
    }))
}

