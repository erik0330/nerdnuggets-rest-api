use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    BidMilestoneSubmissionStatus, BidStatus, BountyDifficulty, BountyReviewType, BountyStatus,
};

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyCreateRequest {
    pub title: String,
    pub description: String,
    pub upload_file: Option<String>,
    pub cover_photo: Option<String>,
    pub category: Vec<Uuid>,
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
pub struct GetBidsOption {
    pub status: Option<BidStatus>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMyBidsOption {
    pub status: Option<BidStatus>,
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
    pub chat_number: String,
    pub receiver_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBountyChatsOption {
    pub chat_number: String,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBountyChatNumbersResponse {
    pub chat_numbers: Vec<String>,
    pub chat_info: Vec<ChatNumberInfo>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatNumberInfo {
    pub chat_number: String,
    pub last_message: String,
    pub last_message_time: Option<DateTime<Utc>>,
    pub unread_count: i32,
    pub bounty: BountyChatBountyInfo,
    pub user: BountyChatUserInfo,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSimilarBountiesOption {
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyChatListResponse {
    pub chat_number: String,
    pub bounty: BountyChatBountyInfo,
    pub funder: BountyChatUserInfo,
    pub created_at: DateTime<Utc>,
    pub last_message: String,
    pub last_message_at: DateTime<Utc>,
    pub unread_count: i32,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyChatBountyInfo {
    pub id: Uuid,
    pub nerd_id: String,
    pub title: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyChatUserInfo {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewBountyWorkSubmissionRequest {
    pub status: BountyReviewType,
    pub admin_notes: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBountyWorkRequest {
    pub title: String,
    pub description: String,
    pub deliverable_files: Vec<String>,
    pub additional_notes: Option<String>,
    pub milestone_submissions: Option<Vec<BountyMilestoneSubmission>>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyMilestoneSubmission {
    pub milestone_number: i16,
    pub title: String,
    pub description: String,
    pub deliverable_files: Vec<String>,
    pub additional_notes: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBidMilestoneWorkRequest {
    pub notes: String,
    pub attached_file_urls: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewBidMilestoneSubmissionRequest {
    pub status: BidMilestoneSubmissionStatus,
    pub feedback: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyActionRequest {
    pub action: BountyAction,
    pub admin_notes: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BountyAction {
    Complete,
    Reject,
}
