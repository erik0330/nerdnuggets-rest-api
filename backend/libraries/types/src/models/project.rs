use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub upload_files: Vec<String>,
    pub cover_photo: Option<String>,
    pub youtube_link: Option<String>,
    pub category: Vec<Uuid>,
    pub status: i16,
    pub funding_goal: i32,
    pub duration: i32,
    pub details: String,
    pub tags: Vec<String>,
    pub team_members: Vec<Uuid>,
    pub milestones: Vec<Uuid>,
    pub analysis: Option<String>,
    pub ai_status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
