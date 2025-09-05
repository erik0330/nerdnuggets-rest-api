use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Milestone, TeamMember, UserInfo};

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Dao {
    pub id: Uuid,
    pub project_id: Uuid,
    pub nerd_id: String,
    pub proposal_id: i64,
    pub user_id: Uuid,
    pub status: i16, // 0: active, 1:success, 2: failed
    pub title: String,
    pub description: String,
    pub details: Option<String>,
    pub funding_goal: i32,
    pub count_for: i32,
    pub count_against: i32,
    pub count_total: i32,
    pub amount_for: i32,
    pub amount_against: i32,
    pub amount_total: i32,
    pub cover_photo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DaoInfo {
    pub id: Uuid,
    pub project_id: Uuid,
    pub nerd_id: String,
    pub proposal_id: i64,
    pub user: UserInfo,
    pub my_vote: Option<MyDaoVote>,
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
    pub cover_photo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DaoDetailInfo {
    pub id: Uuid,
    pub project_id: Uuid,
    pub nerd_id: String,
    pub proposal_id: i64,
    pub user: UserInfo,
    pub my_vote: Option<MyDaoVote>,
    pub status: i16, // 0: active, 1:success, 2: failed
    pub title: String,
    pub description: String,
    pub details: Option<String>,
    pub team_members: Vec<TeamMember>,
    pub milestones: Vec<Milestone>,
    pub funding_goal: i32,
    pub count_for: i32,
    pub count_against: i32,
    pub count_total: i32,
    pub amount_for: i32,
    pub amount_against: i32,
    pub amount_total: i32,
    pub cover_photo: Option<String>,
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
    pub weight: f32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyDaoVote {
    pub status: i16,
    pub weight: f32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct CompletedDao {
    pub id: Uuid,
    pub proposal_id: i64,
    pub created_at: DateTime<Utc>,
}

impl Dao {
    pub fn to_info(&self, user: UserInfo, my_vote: Option<MyDaoVote>) -> DaoInfo {
        DaoInfo {
            id: self.id,
            project_id: self.project_id,
            nerd_id: self.nerd_id.clone(),
            proposal_id: self.proposal_id,
            user,
            my_vote,
            status: self.status,
            title: self.title.clone(),
            description: self.description.clone(),
            funding_goal: self.funding_goal,
            count_for: self.count_for,
            count_against: self.count_against,
            count_total: self.count_total,
            amount_for: self.amount_for,
            amount_against: self.amount_against,
            amount_total: self.amount_total,
            cover_photo: self.cover_photo.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn to_detail_info(
        &self,
        user: UserInfo,
        my_vote: Option<MyDaoVote>,
        team_members: Vec<TeamMember>,
        milestones: Vec<Milestone>,
    ) -> DaoDetailInfo {
        DaoDetailInfo {
            id: self.id,
            project_id: self.project_id,
            nerd_id: self.nerd_id.clone(),
            proposal_id: self.proposal_id,
            user,
            my_vote,
            status: self.status,
            title: self.title.clone(),
            description: self.description.clone(),
            details: self.details.clone(),
            team_members,
            milestones,
            funding_goal: self.funding_goal,
            count_for: self.count_for,
            count_against: self.count_against,
            count_total: self.count_total,
            amount_for: self.amount_for,
            amount_against: self.amount_against,
            amount_total: self.amount_total,
            cover_photo: self.cover_photo.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl DaoVote {
    pub fn my_vote(&self) -> MyDaoVote {
        MyDaoVote {
            status: self.status,
            weight: self.weight,
            comment: self.comment.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
