use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};
use uuid::Uuid;

use crate::models::{Category, UserInfo};

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Bounty {
    pub id: Uuid,
    pub nerd_id: String,
    pub contract_id: i64,
    pub user_id: Uuid,
    pub status: BountyStatus,
    pub title: String,
    pub description: String,
    pub upload_file: Option<String>,
    pub category: Uuid,
    pub difficulty: BountyDifficulty,
    pub tags: Vec<String>,
    pub reward_amount: i32,
    pub reward_currency: String,
    pub deadline: NaiveDate,
    pub requirements: Vec<String>,
    pub deliverables: Vec<String>,
    pub evaluation_criteria: Vec<String>,
    pub by_milestone: bool,
    pub admin_notes: Option<String>,
    pub cancellation_reason: Option<String>,

    pub count_view: i32,
    pub count_comment: i32,
    pub count_bid: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

impl Bounty {
    pub fn to_info(
        &self,
        user: UserInfo,
        category: Option<Category>,
        milestones: Vec<BountyMilestone>,
    ) -> BountyInfo {
        BountyInfo {
            id: self.id,
            nerd_id: self.nerd_id.clone(),
            contract_id: self.contract_id,
            user,
            status: self.status,
            title: self.title.clone(),
            description: self.description.clone(),
            upload_file: self.upload_file.clone(),
            category,
            difficulty: self.difficulty,
            tags: self.tags.clone(),
            reward_amount: self.reward_amount,
            reward_currency: self.reward_currency.clone(),
            deadline: self.deadline.format("%m/%d/%Y").to_string(),
            requirements: self.requirements.clone(),
            deliverables: self.deliverables.clone(),
            evaluation_criteria: self.evaluation_criteria.clone(),
            by_milestone: self.by_milestone,
            milestones,
            admin_notes: self.admin_notes.clone(),
            cancellation_reason: self.cancellation_reason.clone(),
            count_view: self.count_view,
            count_comment: self.count_comment,
            count_bid: self.count_bid,
            created_at: self.created_at,
            updated_at: self.updated_at,
            approved_at: self.approved_at,
            rejected_at: self.rejected_at,
            canceled_at: self.canceled_at,
            started_at: self.started_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyInfo {
    pub id: Uuid,
    pub nerd_id: String,
    pub contract_id: i64,
    pub user: UserInfo,
    pub status: BountyStatus,
    pub title: String,
    pub description: String,
    pub upload_file: Option<String>,
    pub category: Option<Category>,
    pub difficulty: BountyDifficulty,
    pub tags: Vec<String>,
    pub reward_amount: i32,
    pub reward_currency: String,
    pub deadline: String,
    pub requirements: Vec<String>,
    pub deliverables: Vec<String>,
    pub evaluation_criteria: Vec<String>,
    pub by_milestone: bool,
    pub milestones: Vec<BountyMilestone>,
    pub admin_notes: Option<String>,
    pub cancellation_reason: Option<String>,
    pub count_view: i32,
    pub count_comment: i32,
    pub count_bid: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Bid {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub nerd_id: String,
    pub contract_id: i64,
    pub user_id: Uuid,
    pub status: BidStatus,
    pub title: String,
    pub description: String,
    pub bid_amount: i32,
    pub timeline: String,
    pub technical_approach: String,
    pub relevant_experience: String,
    pub budget_breakdown: String,
    pub upload_files: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BountyMilestone {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub number: i16,
    pub title: String,
    pub description: String,
    pub reward_amount: i32,
    pub timeline: Option<String>,
    pub requirements: Vec<String>,
    pub deliverables: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BountyStatus {
    PendingApproval,
    Open,
    Rejected,
    InProgress,
    UnderReview,
    Completed,
    Cancelled,
}

impl From<BountyStatus> for i16 {
    fn from(gender: BountyStatus) -> Self {
        match gender {
            BountyStatus::PendingApproval => 0,
            BountyStatus::Open => 1,
            BountyStatus::Rejected => 2,
            BountyStatus::InProgress => 3,
            BountyStatus::UnderReview => 4,
            BountyStatus::Completed => 5,
            BountyStatus::Cancelled => 6,
        }
    }
}

impl TryFrom<i16> for BountyStatus {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BountyStatus::PendingApproval),
            1 => Ok(BountyStatus::Open),
            2 => Ok(BountyStatus::Rejected),
            3 => Ok(BountyStatus::InProgress),
            4 => Ok(BountyStatus::UnderReview),
            5 => Ok(BountyStatus::Completed),
            6 => Ok(BountyStatus::Cancelled),
            _ => Err("Invalid value for BountyStatus"),
        }
    }
}

impl Type<Postgres> for BountyStatus {
    fn type_info() -> PgTypeInfo {
        <i16 as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for BountyStatus {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        let val: i16 = (*self).into();
        <i16 as sqlx::Encode<'_, sqlx::Postgres>>::encode_by_ref(&val, buf)
    }
}

impl<'r> Decode<'r, Postgres> for BountyStatus {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let val = <i16 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        BountyStatus::try_from(val).map_err(|e| e.into())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BountyDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl From<BountyDifficulty> for i16 {
    fn from(gender: BountyDifficulty) -> Self {
        match gender {
            BountyDifficulty::Beginner => 0,
            BountyDifficulty::Intermediate => 1,
            BountyDifficulty::Advanced => 2,
            BountyDifficulty::Expert => 3,
        }
    }
}

impl TryFrom<i16> for BountyDifficulty {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BountyDifficulty::Beginner),
            1 => Ok(BountyDifficulty::Intermediate),
            2 => Ok(BountyDifficulty::Advanced),
            3 => Ok(BountyDifficulty::Expert),
            _ => Err("Invalid value for BountyDifficulty"),
        }
    }
}

impl Type<Postgres> for BountyDifficulty {
    fn type_info() -> PgTypeInfo {
        <i16 as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for BountyDifficulty {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        let val: i16 = (*self).into();
        <i16 as sqlx::Encode<'_, sqlx::Postgres>>::encode_by_ref(&val, buf)
    }
}

impl<'r> Decode<'r, Postgres> for BountyDifficulty {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let val = <i16 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        BountyDifficulty::try_from(val).map_err(|e| e.into())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BidStatus {
    Submitted,
    UnderReview,
    Accepted,
    Rejected,
    InProgress,
    Completed,
    Cancelled,
}

impl From<BidStatus> for i16 {
    fn from(gender: BidStatus) -> Self {
        match gender {
            BidStatus::Submitted => 0,
            BidStatus::UnderReview => 1,
            BidStatus::Accepted => 2,
            BidStatus::Rejected => 3,
            BidStatus::InProgress => 4,
            BidStatus::Completed => 5,
            BidStatus::Cancelled => 6,
        }
    }
}

impl TryFrom<i16> for BidStatus {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BidStatus::Submitted),
            1 => Ok(BidStatus::UnderReview),
            2 => Ok(BidStatus::Accepted),
            3 => Ok(BidStatus::Rejected),
            4 => Ok(BidStatus::InProgress),
            5 => Ok(BidStatus::Completed),
            6 => Ok(BidStatus::Cancelled),
            _ => Err("Invalid value for BidStatus"),
        }
    }
}

impl Type<Postgres> for BidStatus {
    fn type_info() -> PgTypeInfo {
        <i16 as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for BidStatus {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        let val: i16 = (*self).into();
        <i16 as sqlx::Encode<'_, sqlx::Postgres>>::encode_by_ref(&val, buf)
    }
}

impl<'r> Decode<'r, Postgres> for BidStatus {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let val = <i16 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        BidStatus::try_from(val).map_err(|e| e.into())
    }
}
