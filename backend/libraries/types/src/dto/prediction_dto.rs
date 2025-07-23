use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::PredictionStatus;

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionsOption {
    pub title: Option<String>,
    pub status: Option<PredictionStatus>,
    pub category_id: Option<Uuid>,
    pub is_mine: Option<bool>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMyPredictionStatsResponse {
    pub total_invested: i32,
    pub potential: i32,
    pub active_bets: i32,
    pub accuracy_rate: i32,
}
