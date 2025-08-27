use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct PredictionPlacement {
    pub id: Uuid,
    pub user_address: String,
    pub proposal_id: i64,
    pub milestone_index: i64,
    pub predicts_success: bool,
    pub nerd_amount: i64,
    pub block_number: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PredictionPlacementInfo {
    pub id: Uuid,
    pub user_address: String,
    pub proposal_id: i64,
    pub milestone_index: i64,
    pub predicts_success: bool,
    pub nerd_amount: i64,
    pub block_number: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PredictionPlacement {
    pub fn to_info(&self) -> PredictionPlacementInfo {
        PredictionPlacementInfo {
            id: self.id,
            user_address: self.user_address.clone(),
            proposal_id: self.proposal_id,
            milestone_index: self.milestone_index,
            predicts_success: self.predicts_success,
            nerd_amount: self.nerd_amount,
            block_number: self.block_number,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
