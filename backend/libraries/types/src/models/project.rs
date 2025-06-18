use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserInfo;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Project {
    pub id: Uuid,
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

    pub team_members: Vec<Uuid>,
    pub milestones: Vec<Uuid>,

    pub ai_analysis: Option<String>,
    pub ai_status: Option<i16>,
    pub ai_objectives: Option<i16>,
    pub ai_methodology: Option<i16>,
    pub ai_budget: Option<i16>,
    pub ai_expertise: Option<i16>,
    pub ai_innovation: Option<i16>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn to_info(
        &self,
        user: UserInfo,
        team_members: Vec<TeamMember>,
        milestones: Vec<Milestone>,
    ) -> ProjectInfo {
        ProjectInfo {
            id: self.id,
            user,
            title: self.title.clone(),
            description: self.description.clone(),
            manuscript: self.manuscript.clone(),
            upload_files: self.upload_files.clone(),
            cover_photo: self.cover_photo.clone(),
            youtube_link: self.youtube_link.clone(),
            category: self.category.clone(),
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
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub id: Uuid,
    pub user: UserInfo,
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

    pub team_members: Vec<TeamMember>,
    pub milestones: Vec<Milestone>,

    pub ai_analysis: Option<String>,
    pub ai_status: Option<i16>,
    pub ai_objectives: Option<i16>,
    pub ai_methodology: Option<i16>,
    pub ai_budget: Option<i16>,
    pub ai_expertise: Option<i16>,
    pub ai_innovation: Option<i16>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    pub id: Uuid,
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
    pub title: String,
    pub description: String,
    pub funding_amount: i32,
    pub days_after_start: i32,
    pub days_of_prediction: i32,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: Uuid,
    pub category_name: String,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
