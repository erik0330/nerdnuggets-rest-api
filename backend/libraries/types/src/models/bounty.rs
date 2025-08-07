use crate::models::{Category, UserInfo};
use chrono::{DateTime, NaiveDate, Utc};
use postgres_macro::define_pg_enum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Bounty {
    pub id: Uuid,
    pub nerd_id: String,
    pub contract_id: i64,
    pub user_id: Uuid,
    pub status: BountyStatus,
    pub title: String,
    pub description: String,
    pub upload_file: Option<String>,
    pub cover_photo: Option<String>,
    pub category: Uuid,
    pub difficulty: BountyDifficulty,
    pub tags: Vec<String>,
    pub reward_amount: i32,
    pub reward_currency: String,
    pub deadline: NaiveDate,
    pub requirements: Vec<String>,
    pub deliverables: Vec<String>,
    pub evaluation_criteria: Vec<String>,
    pub by_milestone: bool,
    pub admin_notes: Option<String>,
    pub cancellation_reason: Option<String>,
    pub arweave_tx_id: Option<String>,

    pub count_view: i32,
    pub count_comment: i32,
    pub count_bid: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyInfo {
    pub id: Uuid,
    pub nerd_id: String,
    pub contract_id: i64,
    pub user: UserInfo,
    pub status: BountyStatus,
    pub title: String,
    pub description: String,
    pub upload_file: Option<String>,
    pub cover_photo: Option<String>,
    pub category: Option<Category>,
    pub difficulty: BountyDifficulty,
    pub tags: Vec<String>,
    pub reward_amount: i32,
    pub reward_currency: String,
    pub deadline: String,
    pub requirements: Vec<String>,
    pub deliverables: Vec<String>,
    pub evaluation_criteria: Vec<String>,
    pub by_milestone: bool,
    pub milestones: Vec<BountyMilestone>,
    pub admin_notes: Option<String>,
    pub cancellation_reason: Option<String>,
    pub arweave_tx_id: Option<String>,
    pub count_view: i32,
    pub count_comment: i32,
    pub count_bid: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Bid {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub nerd_id: String,
    pub user_id: Uuid,
    pub status: BidStatus,
    pub title: String,
    pub description: String,
    pub bid_amount: i32,
    pub timeline: String,
    pub technical_approach: String,
    pub relevant_experience: Option<String>,
    pub budget_breakdown: Option<String>,
    pub upload_files: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BidInfo {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub nerd_id: String,
    pub user: UserInfo,
    pub status: BidStatus,
    pub title: String,
    pub description: String,
    pub bid_amount: i32,
    pub timeline: String,
    pub technical_approach: String,
    pub relevant_experience: Option<String>,
    pub budget_breakdown: Option<String>,
    pub upload_files: Vec<String>,
    pub milestones: Vec<BidMilestone>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyMilestone {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub number: i16,
    pub title: String,
    pub description: String,
    pub reward_amount: i32,
    pub timeline: Option<String>,
    pub requirements: Vec<String>,
    pub deliverables: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BidMilestone {
    pub id: Uuid,
    pub bid_id: Uuid,
    pub bounty_id: Uuid,
    pub nerd_id: String,
    pub number: i16,
    pub title: String,
    pub description: String,
    pub amount: i32,
    pub timeline: String,
    pub status: BidMilestoneStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct BountyComment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bounty_id: Uuid,
    pub nerd_id: String,
    pub comment: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyCommentInfo {
    pub id: Uuid,
    pub user: UserInfo,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct BountyChat {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub bounty_id: Uuid,
    pub nerd_id: String,
    pub chat_number: String,
    pub message: String,
    pub file_urls: Vec<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyChatInfo {
    pub id: Uuid,
    pub sender: UserInfo,
    pub receiver: UserInfo,
    pub message: String,
    pub file_urls: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyWorkSubmission {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub bid_id: Uuid,
    pub nerd_id: String,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub deliverable_files: Vec<String>,
    pub additional_notes: Option<String>,
    pub status: BountySubmissionStatus,
    pub admin_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyWorkSubmissionInfo {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub bid_id: Uuid,
    pub nerd_id: String,
    pub user: UserInfo,
    pub title: String,
    pub description: String,
    pub deliverable_files: Vec<String>,
    pub additional_notes: Option<String>,
    pub status: BountySubmissionStatus,
    pub admin_notes: Option<String>,
    pub milestone_submissions: Vec<BountyMilestoneSubmissionInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyMilestoneSubmission {
    pub id: Uuid,
    pub work_submission_id: Uuid,
    pub milestone_number: i16,
    pub title: String,
    pub description: String,
    pub deliverable_files: Vec<String>,
    pub additional_notes: Option<String>,
    pub status: BountySubmissionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyMilestoneSubmissionInfo {
    pub id: Uuid,
    pub work_submission_id: Uuid,
    pub milestone_number: i16,
    pub title: String,
    pub description: String,
    pub deliverable_files: Vec<String>,
    pub additional_notes: Option<String>,
    pub status: BountySubmissionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BountyStatus {
    PendingApproval,
    Open,
    Rejected,
    InProgress,
    UnderReview,
    Completed,
    Cancelled,
    RequestRevision,
}

define_pg_enum!(BountyStatus {
    PendingApproval = 0,
    Open = 1,
    Rejected = 2,
    InProgress = 3,
    UnderReview = 4,
    Completed = 5,
    Cancelled = 6,
    RequestRevision = 7,
});

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BountyReviewType {
    Approve,
    RequestRevision,
    Reject,
}

define_pg_enum!(BountyReviewType {
    Approve = 0,
    RequestRevision = 1,
    Reject = 2,
});

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BountyDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

define_pg_enum!(BountyDifficulty {
    Beginner = 0,
    Intermediate = 1,
    Advanced = 2,
    Expert = 3,
});

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BidStatus {
    Submitted,
    UnderReview,
    Accepted,
    Rejected,
    InProgress,
    Completed,
    Cancelled,
}

define_pg_enum!(BidStatus {
    Submitted = 0,
    UnderReview = 1,
    Accepted = 2,
    Rejected = 3,
    InProgress = 4,
    Completed = 5,
    Cancelled = 6,
});

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BountySubmissionStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    RequestRevision,
}

define_pg_enum!(BountySubmissionStatus {
    Draft = 0,
    Submitted = 1,
    UnderReview = 2,
    Approved = 3,
    Rejected = 4,
    RequestRevision = 5,
});

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BidMilestoneStatus {
    Pending,
    InProgress,
    Submitted,
    Completed,
    Rejected,
}

define_pg_enum!(BidMilestoneStatus {
    Pending = 0,
    InProgress = 1,
    Submitted = 2,
    Completed = 3,
    Rejected = 4,
});

impl Bounty {
    pub fn to_info(
        &self,
        user: UserInfo,
        category: Option<Category>,
        milestones: Vec<BountyMilestone>,
    ) -> BountyInfo {
        BountyInfo {
            id: self.id,
            nerd_id: self.nerd_id.clone(),
            contract_id: self.contract_id,
            user,
            status: self.status,
            title: self.title.clone(),
            description: self.description.clone(),
            upload_file: self.upload_file.clone(),
            cover_photo: self.cover_photo.clone(),
            category,
            difficulty: self.difficulty,
            tags: self.tags.clone(),
            reward_amount: self.reward_amount,
            reward_currency: self.reward_currency.clone(),
            deadline: self.deadline.format("%m/%d/%Y").to_string(),
            requirements: self.requirements.clone(),
            deliverables: self.deliverables.clone(),
            evaluation_criteria: self.evaluation_criteria.clone(),
            by_milestone: self.by_milestone,
            milestones,
            admin_notes: self.admin_notes.clone(),
            cancellation_reason: self.cancellation_reason.clone(),
            arweave_tx_id: self.arweave_tx_id.clone(),
            count_view: self.count_view,
            count_comment: self.count_comment,
            count_bid: self.count_bid,
            created_at: self.created_at,
            updated_at: self.updated_at,
            approved_at: self.approved_at,
            rejected_at: self.rejected_at,
            canceled_at: self.canceled_at,
            started_at: self.started_at,
        }
    }
}

impl Bid {
    pub fn to_info(&self, user: UserInfo, milestones: Vec<BidMilestone>) -> BidInfo {
        BidInfo {
            id: self.id,
            bounty_id: self.bounty_id,
            nerd_id: self.nerd_id.clone(),
            user,
            status: self.status,
            title: self.title.clone(),
            description: self.description.clone(),
            bid_amount: self.bid_amount,
            timeline: self.timeline.clone(),
            technical_approach: self.technical_approach.clone(),
            relevant_experience: self.relevant_experience.clone(),
            budget_breakdown: self.budget_breakdown.clone(),
            upload_files: self.upload_files.clone(),
            milestones,
            created_at: self.created_at,
            updated_at: self.updated_at,
            accepted_at: self.accepted_at,
            rejected_at: self.rejected_at,
            canceled_at: self.canceled_at,
            completed_at: self.completed_at,
        }
    }
}

impl BountyComment {
    pub fn to_info(&self, user: UserInfo) -> BountyCommentInfo {
        BountyCommentInfo {
            id: self.id,
            user,
            comment: self.comment.clone(),
            created_at: self.created_at,
        }
    }
}

impl BountyChat {
    pub fn to_info(&self, sender: UserInfo, receiver: UserInfo) -> BountyChatInfo {
        BountyChatInfo {
            id: self.id,
            sender,
            receiver,
            message: self.message.clone(),
            file_urls: self.file_urls.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl BountyWorkSubmission {
    pub fn to_info(
        &self,
        user: UserInfo,
        milestone_submissions: Vec<BountyMilestoneSubmissionInfo>,
    ) -> BountyWorkSubmissionInfo {
        BountyWorkSubmissionInfo {
            id: self.id,
            bounty_id: self.bounty_id,
            bid_id: self.bid_id,
            nerd_id: self.nerd_id.clone(),
            user,
            title: self.title.clone(),
            description: self.description.clone(),
            deliverable_files: self.deliverable_files.clone(),
            additional_notes: self.additional_notes.clone(),
            status: self.status,
            admin_notes: self.admin_notes.clone(),
            milestone_submissions,
            created_at: self.created_at,
            updated_at: self.updated_at,
            submitted_at: self.submitted_at,
            reviewed_at: self.reviewed_at,
            approved_at: self.approved_at,
            rejected_at: self.rejected_at,
        }
    }
}

impl BountyMilestoneSubmission {
    pub fn to_info(&self) -> BountyMilestoneSubmissionInfo {
        BountyMilestoneSubmissionInfo {
            id: self.id,
            work_submission_id: self.work_submission_id,
            milestone_number: self.milestone_number,
            title: self.title.clone(),
            description: self.description.clone(),
            deliverable_files: self.deliverable_files.clone(),
            additional_notes: self.additional_notes.clone(),
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            submitted_at: self.submitted_at,
            reviewed_at: self.reviewed_at,
            approved_at: self.approved_at,
            rejected_at: self.rejected_at,
        }
    }
}
