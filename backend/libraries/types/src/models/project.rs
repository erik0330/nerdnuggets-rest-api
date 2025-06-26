use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserInfo;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Project {
    pub id: Uuid,
    pub nerd_id: String,
    pub proposal_id: i64,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub manuscript: Option<String>,
    pub upload_files: Vec<String>,
    pub cover_photo: Option<String>,
    pub youtube_link: Option<String>,
    pub category: Vec<Uuid>,
    pub status: i16,
    pub funding_goal: Option<i32>,
    pub duration: Option<i32>,

    pub details: Option<String>,
    pub personnel_cost: Option<i32>,
    pub equipment_cost: Option<i32>,
    pub materials_cost: Option<i32>,
    pub overhead_cost: Option<i32>,
    pub other_cost: Option<i32>,
    pub tags: Vec<String>,

    pub ai_analysis: Option<String>,
    pub ai_status: Option<i16>,
    pub ai_objectives: Option<i16>,
    pub ai_methodology: Option<i16>,
    pub ai_budget: Option<i16>,
    pub ai_expertise: Option<i16>,
    pub ai_innovation: Option<i16>,

    pub funding_amount: i32,
    pub count_contributors: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dao_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

impl Project {
    pub fn to_info(
        &self,
        user: UserInfo,
        category: Vec<Category>,
        team_members: Vec<TeamMember>,
        milestones: Vec<Milestone>,
    ) -> ProjectInfo {
        ProjectInfo {
            id: self.id,
            nerd_id: self.nerd_id.clone(),
            user,
            title: self.title.clone(),
            description: self.description.clone(),
            manuscript: self.manuscript.clone(),
            upload_files: self.upload_files.clone(),
            cover_photo: self.cover_photo.clone(),
            youtube_link: self.youtube_link.clone(),
            category,
            status: self.status,
            funding_goal: self.funding_goal,
            duration: self.duration,
            details: self.details.clone(),
            personnel_cost: self.personnel_cost,
            equipment_cost: self.equipment_cost,
            materials_cost: self.materials_cost,
            overhead_cost: self.overhead_cost,
            other_cost: self.other_cost,
            tags: self.tags.clone(),
            team_members,
            milestones,
            ai_analysis: self.ai_analysis.clone(),
            ai_status: self.ai_status,
            ai_objectives: self.ai_objectives,
            ai_methodology: self.ai_methodology,
            ai_budget: self.ai_budget,
            ai_expertise: self.ai_expertise,
            ai_innovation: self.ai_innovation,
            funding_amount: self.funding_amount,
            count_contributors: self.count_contributors,
            created_at: self.created_at,
            updated_at: self.updated_at,
            dao_at: self.dao_at,
            started_at: self.started_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub id: Uuid,
    pub nerd_id: String,
    pub user: UserInfo,
    pub title: Option<String>,
    pub description: Option<String>,
    pub manuscript: Option<String>,
    pub upload_files: Vec<String>,
    pub cover_photo: Option<String>,
    pub youtube_link: Option<String>,
    pub category: Vec<Category>,
    pub status: i16,
    pub funding_goal: Option<i32>,
    pub duration: Option<i32>,

    pub details: Option<String>,
    pub personnel_cost: Option<i32>,
    pub equipment_cost: Option<i32>,
    pub materials_cost: Option<i32>,
    pub overhead_cost: Option<i32>,
    pub other_cost: Option<i32>,
    pub tags: Vec<String>,

    pub team_members: Vec<TeamMember>,
    pub milestones: Vec<Milestone>,

    pub ai_analysis: Option<String>,
    pub ai_status: Option<i16>,
    pub ai_objectives: Option<i16>,
    pub ai_methodology: Option<i16>,
    pub ai_budget: Option<i16>,
    pub ai_expertise: Option<i16>,
    pub ai_innovation: Option<i16>,

    pub funding_amount: i32,
    pub count_contributors: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dao_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub role: String,
    pub bio: String,
    pub linkedin: String,
    pub twitter: String,
    pub github: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    pub id: Uuid,
    pub project_id: Uuid,
    pub status: i16, // 0:pending, 1:in process, 2:success, 3:giveup, 4:failed
    pub number: i16,
    pub title: String,
    pub description: String,
    pub deliverables: Option<String>,
    pub challenges: Option<String>,
    pub next_steps: Option<String>,
    pub file_urls: Vec<String>,
    pub proof_status: i16, // 0:empty, 1:submitted, 2:approved, 3:rejected
    pub funding_amount: i32,
    pub days_after_start: i32,
    pub days_of_prediction: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectItem {
    pub id: Uuid,
    pub nerd_id: String,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub cover_photo: Option<String>,
    pub category: Vec<Uuid>,
    pub status: i16,
    pub funding_goal: Option<i32>,
    pub duration: Option<i32>,
    pub tags: Vec<String>,

    pub funding_amount: i32,
    pub count_contributors: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dao_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

impl ProjectItem {
    pub fn to_info(
        &self,
        user: UserInfo,
        editor: Option<ProjectEditorInfo>,
        category: Vec<Category>,
    ) -> ProjectItemInfo {
        ProjectItemInfo {
            id: self.id,
            nerd_id: self.nerd_id.clone(),
            user,
            editor,
            title: self.title.clone().unwrap_or_default(),
            description: self.description.clone().unwrap_or_default(),
            cover_photo: self.cover_photo.clone(),
            category,
            status: self.status,
            funding_goal: self.funding_goal,
            duration: self.duration,
            tags: self.tags.clone(),
            funding_amount: self.funding_amount,
            count_contributors: self.count_contributors,
            created_at: self.created_at,
            updated_at: self.updated_at,
            dao_at: self.dao_at,
            started_at: self.started_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectItemInfo {
    pub id: Uuid,
    pub nerd_id: String,
    pub user: UserInfo,
    pub editor: Option<ProjectEditorInfo>,
    pub title: String,
    pub description: String,
    pub cover_photo: Option<String>,
    pub category: Vec<Category>,
    pub status: i16,
    pub funding_goal: Option<i32>,
    pub duration: Option<i32>,
    pub tags: Vec<String>,

    pub funding_amount: i32,
    pub count_contributors: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dao_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct ProjectEditor {
    pub id: Uuid,
    pub project_id: String,
    pub nerd_id: String,
    pub user_id: Uuid,
    pub status: i16, // FeedbackStatus => 0: pending, 1: accepted, 2: request revision, 3: rejected
    pub feedback: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProjectEditor {
    pub fn to_info(&self, is_full: bool, user: UserInfo) -> ProjectEditorInfo {
        ProjectEditorInfo {
            id: self.id,
            user,
            status: self.status,
            feedback: self.feedback.clone().filter(|_| is_full),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectEditorInfo {
    pub id: Uuid,
    pub user: UserInfo,
    pub status: i16, // FeedbackStatus => 0: pending, 1: accepted, 2: request revision, 3: rejected
    pub feedback: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIds {
    pub id: Uuid,
    pub nerd_id: String,
}
