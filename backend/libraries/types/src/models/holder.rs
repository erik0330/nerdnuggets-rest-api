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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
