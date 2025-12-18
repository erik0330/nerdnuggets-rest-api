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
