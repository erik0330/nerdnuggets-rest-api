use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::RewardType;

#[derive(Clone, Deserialize, Serialize, Validate, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardCreateRequest {
    pub user_id: Option<Uuid>,
    pub wallet_address: String,
    pub reward_type: RewardType,
    pub points: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardUpdateRequest {
    pub points: i32,
    pub amounts: i32,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub reward_type: String,
    pub points: i32,
    pub amounts: i32,
    pub claimed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardStatsResponse {
    pub total_points: i32,
    pub total_amounts: i32,
    pub unclaimed_rewards: i32,
    pub claimed_rewards: i32,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardListResponse {
    pub rewards: Vec<RewardResponse>,
    pub total: i64,
    pub offset: i32,
    pub limit: i32,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardTypeResponse {
    pub reward_type: String,
    pub description: String,
    pub points_multiplier: f32,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardAwardRequest {
    pub wallet_address: String,
    pub points: i32,
    pub amounts: i32,
}

// Validation for reward types
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum RewardTypeEnum {
    StartDao,
    FundingProcess,
    Fund,
    Predict,
}

impl std::fmt::Display for RewardTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RewardTypeEnum::StartDao => write!(f, "StartDao"),
            RewardTypeEnum::FundingProcess => write!(f, "FundingProcess"),
            RewardTypeEnum::Fund => write!(f, "Fund"),
            RewardTypeEnum::Predict => write!(f, "Predict"),
        }
    }
}

impl std::str::FromStr for RewardTypeEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "StartDao" => Ok(RewardTypeEnum::StartDao),
            "FundingProcess" => Ok(RewardTypeEnum::FundingProcess),
            "Fund" => Ok(RewardTypeEnum::Fund),
            "Predict" => Ok(RewardTypeEnum::Predict),
            _ => Err(format!("Invalid reward type: {}", s)),
        }
    }
}
