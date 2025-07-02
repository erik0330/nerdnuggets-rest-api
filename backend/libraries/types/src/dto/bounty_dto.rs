use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::BountyDifficulty;

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyCreateRequest {
    pub title: String,
    pub description: String,
    pub upload_file: Option<String>,
    pub category: Uuid,
    pub difficulty: BountyDifficulty,
    pub tags: Option<Vec<String>>,
    pub reward_amount: i32,
    pub reward_currency: String,
    pub deadline: String,
    pub requirements: Vec<String>,
    pub deliverables: Vec<String>,
    pub evaluation_criteria: Vec<String>,
    pub by_milestone: bool,
    pub milestones: Option<Vec<BountyMilestoneRequest>>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyMilestoneRequest {
    pub number: i16,
    pub title: String,
    pub description: String,
    pub reward_amount: i32,
    pub timeline: Option<String>,
    pub requirements: Option<Vec<String>>,
    pub deliverables: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBountysOption {
    pub title: Option<String>,
    pub status: Option<i16>,
    pub category_id: Option<Uuid>,
    pub is_mine: Option<bool>,
    pub is_public: Option<bool>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

// #[derive(Clone, Serialize, Deserialize, Validate, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct MakeDecisionRequest {
//     pub status: i16,
//     pub to_dao: Option<bool>,
//     pub feedback: Option<String>,
// }

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBountyCommentsOption {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBountyCommentRequest {
    pub comment: String,
}
