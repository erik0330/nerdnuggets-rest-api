pub mod analysis_error;
pub mod dto;
pub mod error;
pub mod models;
pub mod research_check;
pub mod response;

mod subscription;

use std::fmt;

pub use subscription::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type TimestampMillis = u64;

#[derive(sqlx::FromRow)]
pub struct InsertResult {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EmailVerifyType {
    AddEmail,
    VerifyEmail,
    ResetPassword,
}

impl std::fmt::Display for EmailVerifyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            EmailVerifyType::AddEmail => "AddEmail",
            EmailVerifyType::VerifyEmail => "VerifyEmail",
            EmailVerifyType::ResetPassword => "ResetPassword",
        };
        f.write_str(name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum PostType {
    IntroduceYourself,
    UserFeedback,
    Job,
    GeneralDiscussion,
    Questions,
    Events,
    ResearchCheck,
    GenerateArticle,
    Article,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShowerType {
    SpecificUsers,
    MyFollowers,
    Everyone,
    Nobody,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum AIErrorCheckStatus {
    #[default]
    NotYet,
    InProcess,
    Done,
    Error,
}

#[allow(dead_code)]
impl AIErrorCheckStatus {
    pub fn from(status: i16) -> Self {
        match status {
            0 => Self::NotYet,
            1 => Self::InProcess,
            2 => Self::Done,
            3 | _ => Self::Error,
        }
    }
    pub fn to_i16(&self) -> i16 {
        self.to_owned() as i16
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ProjectStatus {
    #[default]
    Creating,
    PendingReview,
    UnderReview,
    RevisionEditor,
    ApprovedEditor,
    RevisionAdmin,
    ApprovedAdmin,
    DaoVoting,
    Funding,
    Completed,
    Rejected,
}

#[allow(dead_code)]
impl ProjectStatus {
    pub fn from(status: i16) -> Self {
        match status {
            0 => Self::Creating,
            1 => Self::PendingReview,
            2 => Self::UnderReview,
            3 => Self::RevisionEditor,
            4 => Self::ApprovedEditor,
            5 => Self::RevisionAdmin,
            6 => Self::ApprovedAdmin,
            7 => Self::DaoVoting,
            8 => Self::Funding,
            9 => Self::Completed,
            10 | _ => Self::Rejected,
        }
    }
    pub fn to_i16(&self) -> i16 {
        self.to_owned() as i16
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum MileStoneStatus {
    #[default]
    Pending,
    InProgress,
    Success,
    GiveUp,
    Failed,
}

#[allow(dead_code)]
impl MileStoneStatus {
    pub fn from(status: i16) -> Self {
        match status {
            0 => Self::Pending,
            1 => Self::InProgress,
            2 => Self::Success,
            3 => Self::GiveUp,
            4 | _ => Self::Failed,
        }
    }
    pub fn to_i16(&self) -> i16 {
        self.to_owned() as i16
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ProjectResultStatus {
    #[default]
    Empty,
    Submitted,
    Approved,
    Rejected,
}

impl ProjectResultStatus {
    pub fn from(status: i16) -> Self {
        match status {
            0 => Self::Empty,
            1 => Self::Submitted,
            2 => Self::Approved,
            3 | _ => Self::Rejected,
        }
    }
    pub fn to_i16(&self) -> i16 {
        self.to_owned() as i16
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum FeedbackStatus {
    #[default]
    Pending,
    Accepted,
    RevisionRequired,
    Rejected,
}

#[allow(dead_code)]
impl FeedbackStatus {
    pub fn from(status: i16) -> Self {
        match status {
            0 => Self::Pending,
            1 => Self::Accepted,
            2 => Self::RevisionRequired,
            3 | _ => Self::Rejected,
        }
    }
    pub fn to_i16(&self) -> i16 {
        self.to_owned() as i16
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum UserRoleType {
    #[default]
    Researcher,
    Editor,
    Admin,
    Funder,
    Predictor,
    Member,
    Student,
}

impl fmt::Display for UserRoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UserRoleType::Researcher => "Researcher",
            UserRoleType::Editor => "Editor",
            UserRoleType::Admin => "Admin",
            UserRoleType::Funder => "Funder",
            UserRoleType::Predictor => "Predictor",
            UserRoleType::Member => "Member",
            UserRoleType::Student => "Student",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum UserTierType {
    #[default]
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

impl fmt::Display for UserTierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UserTierType::Bronze => "Bronze",
            UserTierType::Silver => "Silver",
            UserTierType::Gold => "Gold",
            UserTierType::Platinum => "Platinum",
            UserTierType::Diamond => "Diamond",
        };
        write!(f, "{}", s)
    }
}

pub trait PushIfNotContains<T> {
    fn push_if_not_contains(&mut self, item: T) -> bool;
}

impl<T: PartialEq> PushIfNotContains<T> for Vec<T> {
    fn push_if_not_contains(&mut self, item: T) -> bool {
        if !self.contains(&item) {
            self.push(item);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum NerdNuggetsOAuth2AppName {
    NerdBunny,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AnalysisErrorType {
    DataAnalysisError,
    LogicalFrameworkError,
    MathError,
    MethodologyError,
    ResearchQualityError,
    TechnicalPresentationError,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ErrorStatistics {
    pub math_errors: i64,
    pub methodology_errors: i64,
    pub logical_framework_errors: i64,
    pub data_analysis_errors: i64,
    pub technical_presentation_errors: i64,
    pub research_quality_errors: i64,
    pub total_errors: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub total_papers: i64,
    pub total_analyses: i64,
    pub error_statistics: ErrorStatistics,
}
