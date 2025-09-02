use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::DaoInfo;

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdateStep1Request {
    pub manuscript: Option<String>,
    pub upload_files: Option<Vec<String>>,
    pub cover_photo: Option<String>,
    pub title: String,
    pub description: String,
    pub category: Vec<Uuid>,
    pub funding_goal: i32,
    pub duration: i32,
    pub youtube_link: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdateStep2Request {
    pub details: String,
    pub personnel_cost: i32,
    pub equipment_cost: Option<i32>,
    pub materials_cost: Option<i32>,
    pub overhead_cost: Option<i32>,
    pub other_cost: i32,
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdateStep3Request {
    pub team_members: Vec<ProjectTeamMemberRequest>,
    pub milestones: Vec<ProjectMilestoneRequest>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTeamMemberRequest {
    pub name: String,
    pub role: String,
    pub bio: String,
    pub linkedin: String,
    pub twitter: String,
    pub github: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMilestoneRequest {
    pub number: i16,
    pub title: String,
    pub description: String,
    pub funding_amount: i32,
    pub days_after_start: i32,
    pub days_of_prediction: i32,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectsOption {
    pub title: Option<String>,
    pub status: Option<i16>,
    pub category_id: Option<Uuid>,
    pub is_mine: Option<bool>,
    pub is_public: Option<bool>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssignEditorRequest {
    pub editor_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakeDecisionRequest {
    pub status: i16,
    pub to_dao: Option<bool>,
    pub feedback: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMilestoneRequest {
    pub progress_description: String,
    pub deliverables: Option<String>,
    pub challenges: Option<String>,
    pub next_steps: Option<String>,
    pub file_urls: Option<Vec<String>>,
    pub is_draft: bool,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectCommentsOption {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitProjectCommentRequest {
    pub comment: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDaosOption {
    pub title: Option<String>,
    pub status: Option<i16>,
    pub is_mine: Option<bool>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitDaoVoteRequest {
    pub proposal_id: String,
    pub wallet: String,
    pub support: bool,
    pub weight: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSimilarProjectsOption {
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectFundersOption {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatusCount {
    pub status: i16,
    pub count: i64,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatusCounts {
    pub counts: Vec<ProjectStatusCount>,
    pub total: i64,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCountsResponse {
    pub all: i64,
    pub my_project: i64,
    pub funding: i64,
    pub featured: i64,
    pub trending: i64,
    pub funded: i64,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminProjectDashboardCounts {
    pub all: i64,
    pub pending_review: i64,
    pub under_review: i64,
    pub approved: i64,
    pub dao_voting: i64,
    pub needs_revision: i64,
    pub rejected: i64,
    pub funding: i64,
    pub completed: i64,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EditorDashboardCounts {
    pub pending_reviews: i64,
    pub completed_reviews: i64,
    pub total_assigned: i64,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DaoStatisticsResponse {
    pub total: i64,
    pub active: i64,
    pub success: i64,
    pub failed: i64,
}

#[derive(Clone, Deserialize, Serialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneApprovalRequest {
    pub status: MilestoneApprovalStatus,
    pub feedback: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MilestoneApprovalStatus {
    Approved,
    Rejected,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFundersResponse {
    pub funders: Vec<ProjectFunderInfo>,
    pub total_amount: i32,
    pub total_count: i64,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFunderInfo {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub wallet: String,
    pub avatar_url: Option<String>,
    pub number: i16,
    pub amount: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResearchProjectDashboardResponse {
    pub total_projects: i32,
    pub total_funded: i32,
    pub total_backers: i32,
    pub completed: i32,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDaoVotingStats {
    pub total_votes: i64,
    pub yes_votes: i64,
    pub no_votes: i64,
    pub passed: i64,
    pub failed: i64,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub enum DaoVoteTab {
    #[default]
    All,
    Yes,
    No,
    Passed,
    Failed,
}

#[derive(Clone, Deserialize, Serialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDaoVotesOption {
    pub search: Option<String>,
    pub tab: Option<DaoVoteTab>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDaoVotesResponse {
    pub daos: Vec<DaoInfo>,
    pub total: i64,
    pub stats: UserDaoVotingStats,
}
