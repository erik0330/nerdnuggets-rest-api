use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
