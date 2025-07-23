use crate::models::{Category, UserInfo};
use chrono::{DateTime, NaiveDate, Utc};
use postgres_macro::define_pg_enum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Prediction {
    pub id: Uuid,
    pub nerd_id: String,
    pub contract_id: i64,
    pub status: PredictionStatus,
    pub number: i16,
    pub milestone_id: Uuid,
    pub title: String,
    pub description: String,
    pub funding_amount: i32,
    pub pool_amount: i32,
    pub yes_pool_amount: i32,
    pub no_pool_amount: i32,
    pub progress: i16,
    pub count_predictors: i32,
    pub count_view: i32,

    pub project_id: Uuid,
    pub project_nerd_id: String,
    pub proposal_id: i64,
    pub project_title: String,
    pub user_id: Uuid,
    pub cover_photo: Option<String>,
    pub category: Vec<Uuid>,
    pub tags: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: NaiveDate,
    pub ended_at: NaiveDate,
    pub released_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PredictionInfo {
    pub id: Uuid,
    pub nerd_id: String,
    pub status: PredictionStatus,
    pub number: i16,
    pub milestone_id: Uuid,
    pub title: String,
    pub description: String,
    pub funding_amount: i32,
    pub pool_amount: i32,
    pub yes_pool_amount: i32,
    pub no_pool_amount: i32,
    pub progress: i16,
    pub count_predictors: i32,
    pub count_view: i32,

    pub project_id: Uuid,
    pub project_nerd_id: String,
    pub proposal_id: i64,
    pub project_title: String,
    pub user: UserInfo,
    pub cover_photo: Option<String>,
    pub category: Vec<Category>,
    pub tags: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: String,
    pub ended_at: String,
    pub released_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum PredictionStatus {
    Active,
    Upcoming,
    Completed,
    Cancelled,
}

define_pg_enum!(PredictionStatus {
    Active = 0,
    Upcoming = 1,
    Completed = 2,
    Cancelled = 3,
});

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopPredictor {
    pub id: Uuid,
    pub name: String,
    pub count_prediction: i32,
    pub accuracy_rate: i32,
}

impl Prediction {
    pub fn to_info(&self, user: UserInfo, category: Vec<Category>) -> PredictionInfo {
        PredictionInfo {
            id: self.id,
            nerd_id: self.nerd_id.clone(),
            status: self.status,
            number: self.number,
            milestone_id: self.milestone_id,
            title: self.title.clone(),
            description: self.description.clone(),
            funding_amount: self.funding_amount,
            pool_amount: self.pool_amount,
            yes_pool_amount: self.yes_pool_amount,
            no_pool_amount: self.no_pool_amount,
            progress: self.progress,
            count_predictors: self.count_predictors,
            count_view: self.count_view,
            project_id: self.project_id,
            project_nerd_id: self.project_nerd_id.clone(),
            proposal_id: self.proposal_id,
            project_title: self.project_title.clone(),
            user,
            cover_photo: self.cover_photo.clone(),
            category,
            tags: self.tags.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            started_at: self.started_at.format("%m/%d/%Y").to_string(),
            ended_at: self.ended_at.format("%m/%d/%Y").to_string(),
            released_at: self.released_at,
        }
    }
}
