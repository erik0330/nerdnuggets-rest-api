use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Nomination {
    pub id: Uuid,

    // paper info
    pub doi: String,
    pub title: String,
    #[serde(rename = "abstract")]
    #[sqlx(rename = "abstract")]
    pub _abstract: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub journal: String,
    pub keywords: Vec<String>,
    pub status: i16, // 0: fetched, 1: submitted, 2: under_review, 3: approved, 4: rejected, 5: project created NominationStatus

    // nominator info
    pub nominator_id: Option<Uuid>,
    pub nominator_name: Option<String>,
    pub nominator_email: Option<String>,
    pub nominator_institution: Option<String>,
    pub nominator_text: Option<String>,

    // ai info
    pub ai_score: Option<f32>,
    pub ai_analysis: Option<String>,

    // project info
    pub project_id: Option<Uuid>,
    pub identifier: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum NominationStatus {
    #[default]
    Fetched,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    ProjectCreated,
}

impl NominationStatus {
    pub fn from(status: i16) -> Self {
        match status {
            0 => Self::Fetched,
            1 => Self::Submitted,
            2 => Self::UnderReview,
            3 => Self::Approved,
            4 => Self::Rejected,
            5 => Self::ProjectCreated,
            _ => Self::Fetched,
        }
    }
    pub fn to_i16(&self) -> i16 {
        self.to_owned() as i16
    }
}
