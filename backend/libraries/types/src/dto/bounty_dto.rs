use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::{BountyDifficulty, BountyReviewType, BountyStatus};

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyCreateRequest {
    pub title: String,
    pub description: String,
    pub upload_file: Option<String>,
    pub cover_photo: Option<String>,
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
    pub title: String,
    pub description: String,
    pub reward_amount: i32,
    pub timeline: Option<String>,
    pub requirements: Option<Vec<String>>,
    pub deliverables: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyUpdateRequest {
    pub title: String,
    pub description: String,
    pub reward_amount: i32,
    pub reward_currency: String,
    pub difficulty: BountyDifficulty,
    pub deadline: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBountysOption {
    pub title: Option<String>,
    pub status: Option<BountyStatus>,
    pub category_id: Option<Uuid>,
    pub difficulty: Option<BountyDifficulty>,
    pub is_mine: Option<bool>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBidRequest {
    pub title: String,
    pub description: String,
    pub bid_amount: i32,
    pub timeline: String,
    pub technical_approach: String,
    pub relevant_experience: String,
    pub budget_breakdown: String,
    pub milestones: Vec<BidMilestoneRequest>,
    pub upload_files: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BidMilestoneRequest {
    pub title: String,
    pub description: String,
    pub amount: i32,
    pub timeline: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OffsetAndLimitOption {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBountyCommentRequest {
    pub comment: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewBountyRequest {
    pub status: BountyReviewType,
    pub admin_notes: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMyBountyStatsResponse {
    pub total_earned: i32,
    pub completed: i32,
    pub in_progress: i32,
    pub success_rate: i32,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendBountyChatRequest {
    pub message: String,
    pub file_urls: Option<Vec<String>>,
}
