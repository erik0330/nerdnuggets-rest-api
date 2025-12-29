use crate::models::Holder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderDto {
    pub id: Uuid,
    pub telegram_id: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo_url: Option<String>,
    pub total_coins: i64,
    pub current_coins: i64,
    pub referrer_telegram_id: Option<String>,
    pub referral_rewards_earned: i64,
    pub total_referred: i32,
    pub referral_rewards_claimed: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl HolderDto {
    pub fn from(model: Holder) -> HolderDto {
        Self {
            id: model.id,
            telegram_id: model.telegram_id,
            username: model.username,
            first_name: model.first_name,
            last_name: model.last_name,
            photo_url: model.photo_url,
            total_coins: model.total_coins,
            current_coins: model.current_coins,
            referrer_telegram_id: model.referrer_telegram_id,
            referral_rewards_earned: model.referral_rewards_earned,
            total_referred: model.total_referred,
            referral_rewards_claimed: model.referral_rewards_claimed,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldersListResponse {
    pub holders: Vec<HolderDto>,
    pub total_count: i64,
    pub has_more: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelegramLoginRequest {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub auth_date: i64,
    pub hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTelegramResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRequest {
    pub telegram_id: u64,
    pub wallet_address: String,
    pub message: String,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_telegram_id: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_claimed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
