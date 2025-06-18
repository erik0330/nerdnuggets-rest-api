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
