use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, FromRow, Default, Debug)]
pub struct Reward {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub wallet_address: String,
    pub reward_type: String,
    pub points: i32,
    pub amounts: i32,
    pub claimed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardInfo {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub wallet_address: String,
    pub reward_type: String,
    pub points: i32,
    pub amounts: i32,
    pub claimed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Reward {
    pub fn to_info(&self) -> RewardInfo {
        RewardInfo {
            id: self.id,
            user_id: self.user_id,
            wallet_address: self.wallet_address.clone(),
            reward_type: self.reward_type.clone(),
            points: self.points,
            amounts: self.amounts,
            claimed_at: self.claimed_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRewardRequest {
    pub user_id: Uuid,
    pub wallet_address: String,
    pub reward_type: String,
    pub points: i32,
    pub amounts: i32,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRewardRequest {
    pub reward_id: Uuid,
}

#[derive(Clone, Deserialize, Serialize, FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardStats {
    pub total_points: i32,
    pub total_amounts: i32,
    pub unclaimed_rewards: i32,
    pub claimed_rewards: i32,
}

// Reward types enum
#[derive(Clone, Deserialize, Serialize, Default, Debug, PartialEq)]
pub enum RewardType {
    #[default]
    StartDao,
    FundingProcess,
    Fund,
    Predict,
}

impl std::fmt::Display for RewardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RewardType::StartDao => write!(f, "StartDao"),
            RewardType::FundingProcess => write!(f, "FundingProcess"),
            RewardType::Fund => write!(f, "Fund"),
            RewardType::Predict => write!(f, "Predict"),
        }
    }
}

impl std::str::FromStr for RewardType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "StartDao" => Ok(RewardType::StartDao),
            "FundingProcess" => Ok(RewardType::FundingProcess),
            "Fund" => Ok(RewardType::Fund),
            "Predict" => Ok(RewardType::Predict),
            _ => Err(format!("Invalid reward type: {}", s)),
        }
    }
}
