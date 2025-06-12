use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct HashTags {
    pub id: Uuid,
    pub hashtag_name: String,
    pub user_id: Option<Uuid>,
    pub is_available: bool,
    pub usage_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct HashTagsInfo {
    pub id: Uuid,
    pub hashtag_name: String,
}
