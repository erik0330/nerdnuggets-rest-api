use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use types::error::UserError;
use types::UserRoleType;
use types::{
    dto::{OffsetAndLimitOption, RewardCreateRequest},
    error::{ApiError, DbError, ValidatedRequest},
    models::{RewardInfo, RewardStats, User},
};
use utils::commons::uuid_from_str;
use uuid::Uuid;

// Create a new reward
pub async fn create_reward(
    Extension(role): Extension<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<RewardCreateRequest>,
) -> Result<Json<RewardInfo>, ApiError> {
    if role != UserRoleType::Admin.to_string() {
        return Err(ApiError::UserError(UserError::Str(
            "You are not authorized to create rewards".to_string(),
        )));
    }
    let reward = state
        .service
        .reward
        .create_reward(
            payload.user_id,
            &payload.wallet_address,
            payload.reward_type,
            payload.points.unwrap_or(0),
        )
        .await?;
    Ok(Json(reward.to_info()))
}

// Get reward by ID
pub async fn get_reward_by_id(
    Path(reward_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<RewardInfo>, ApiError> {
    let reward_id = Uuid::parse_str(&reward_id)
        .map_err(|_| ApiError::DbError(DbError::Str("Invalid reward ID format".to_string())))?;

    let reward = state.service.reward.get_reward_by_id(reward_id).await?;
    Ok(Json(reward.to_info()))
}

// Get user's rewards
pub async fn get_user_rewards(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<RewardInfo>>, ApiError> {
    let rewards = state
        .service
        .reward
        .get_user_rewards(user.id, opts.offset, opts.limit)
        .await?;
    Ok(Json(rewards))
}

// Get user's unclaimed rewards
pub async fn get_user_unclaimed_rewards(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<RewardInfo>>, ApiError> {
    let rewards = state
        .service
        .reward
        .get_user_unclaimed_rewards(user.id, opts.offset, opts.limit)
        .await?;
    Ok(Json(rewards))
}

// Get user's claimed rewards
pub async fn get_user_claimed_rewards(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<RewardInfo>>, ApiError> {
    let rewards = state
        .service
        .reward
        .get_user_claimed_rewards(user.id, opts.offset, opts.limit)
        .await?;
    Ok(Json(rewards))
}

// Claim a reward
pub async fn claim_reward(
    Extension(user): Extension<User>,
    Path(reward_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<RewardInfo>, ApiError> {
    let reward = state
        .service
        .reward
        .claim_reward(user.id, uuid_from_str(&reward_id)?)
        .await?;
    Ok(Json(reward.to_info()))
}

// Get user's reward statistics
pub async fn get_user_reward_stats(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<RewardStats>, ApiError> {
    let stats = state.service.reward.get_user_reward_stats(user.id).await?;
    Ok(Json(stats))
}

// Get rewards by type
pub async fn get_rewards_by_type(
    Path(reward_type): Path<String>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<RewardInfo>>, ApiError> {
    let rewards = state
        .service
        .reward
        .get_rewards_by_type(&reward_type, opts.offset, opts.limit)
        .await?;
    Ok(Json(rewards))
}

// Get rewards by wallet address
pub async fn get_rewards_by_wallet_address(
    Path(wallet_address): Path<String>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<RewardInfo>>, ApiError> {
    let rewards = state
        .service
        .reward
        .get_rewards_by_wallet_address(&wallet_address, opts.offset, opts.limit)
        .await?;
    Ok(Json(rewards))
}

// Delete a reward (only if it belongs to the user)
pub async fn delete_reward(
    Extension(user): Extension<User>,
    Path(reward_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let success = state
        .service
        .reward
        .delete_reward(user.id, uuid_from_str(&reward_id)?)
        .await?;
    Ok(Json(success))
}

// Update reward points (only if it belongs to the user and is not claimed)
pub async fn update_reward_points(
    Extension(user): Extension<User>,
    Path(reward_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<RewardCreateRequest>,
) -> Result<Json<RewardInfo>, ApiError> {
    let reward_id = uuid_from_str(&reward_id)?;

    let reward = state
        .service
        .reward
        .update_reward_points(user.id, reward_id, payload.points.unwrap_or(0), 0)
        .await?;
    Ok(Json(reward.to_info()))
}

// Get all rewards
pub async fn get_all_rewards(
    Extension(role): Extension<String>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<RewardInfo>>, ApiError> {
    if role != UserRoleType::Admin.to_string() {
        return Err(ApiError::UserError(UserError::Str(
            "You are not authorized to get all rewards".to_string(),
        )));
    }
    let rewards = state
        .service
        .reward
        .get_all_rewards(opts.offset, opts.limit)
        .await?;
    Ok(Json(rewards))
}

// Get rewards count for user
pub async fn get_user_rewards_count(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<i64>, ApiError> {
    let count = state
        .service
        .reward
        .get_rewards_count_by_user(user.id)
        .await?;
    Ok(Json(count))
}

// Get unclaimed rewards count for user
pub async fn get_user_unclaimed_rewards_count(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<i64>, ApiError> {
    let count = state
        .service
        .reward
        .get_unclaimed_rewards_count_by_user(user.id)
        .await?;
    Ok(Json(count))
}
