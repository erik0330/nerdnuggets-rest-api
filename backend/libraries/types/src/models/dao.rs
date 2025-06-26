use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dao {
    pub id: Uuid,
    pub project_id: Uuid,
    pub nerd_id: String,
    pub proposal_id: i64,
    pub user_id: Uuid,
    pub status: i16, // 0: active, 1:success, 2: failed
    pub title: String,
    pub description: String,
    pub funding_goal: i32,
    pub count_for: i32,
    pub count_against: i32,
    pub count_total: i32,
    pub amount_for: i32,
    pub amount_against: i32,
    pub amount_total: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DaoVote {
    pub id: Uuid,
    pub dao_id: Uuid,
    pub project_id: Uuid,
    pub nerd_id: String,
    pub proposal_id: i64,
    pub user_id: Uuid,
    pub status: i16, // 0: empty, 1: for, 2: against
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
