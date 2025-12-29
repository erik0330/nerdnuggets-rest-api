use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Holder {
    pub id: Uuid,
    pub telegram_id: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo_url: Option<String>,
    pub total_coins: i64,
    pub current_coins: i64,
    pub wallet_address: Option<String>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub referrer_telegram_id: Option<String>,
    pub referral_rewards_earned: i64,
    pub total_referred: i32,
    pub referral_rewards_claimed: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
