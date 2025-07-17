use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub enum MessageType {
    #[default]
    One,
    All,
}

impl From<MessageType> for i32 {
    fn from(message_type: MessageType) -> i32 {
        match message_type {
            MessageType::One => 0,
            MessageType::All => 1,
        }
    }
}

impl TryFrom<i32> for MessageType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::One),
            1 => Ok(MessageType::All),
            _ => Err(format!("Invalid value for MessageType: {}", value)),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub enum NotificationType {
    #[default]
    InviteEditor,
    CancelEditor,
    AcceptEditor,
    DeclineEditor,
    NewBounty,
    NewDAO,
    NewPrediction,
    NewMessage,
    ApprovedBid,
    RejectedBid,
    BidReviewed,
    NewProject,
    ProjectMilestone,
    ProjectComment,
    BountyComment,
    DAOVote,
    FundingUpdate,
    PredictionResult,
    SystemMessage,
}

impl From<NotificationType> for i32 {
    fn from(notification_type: NotificationType) -> i32 {
        match notification_type {
            NotificationType::InviteEditor => 0,
            NotificationType::CancelEditor => 1,
            NotificationType::AcceptEditor => 2,
            NotificationType::DeclineEditor => 3,
            NotificationType::NewBounty => 4,
            NotificationType::NewDAO => 5,
            NotificationType::NewPrediction => 6,
            NotificationType::NewMessage => 7,
            NotificationType::ApprovedBid => 8,
            NotificationType::RejectedBid => 9,
            NotificationType::BidReviewed => 10,
            NotificationType::NewProject => 11,
            NotificationType::ProjectMilestone => 12,
            NotificationType::ProjectComment => 13,
            NotificationType::BountyComment => 14,
            NotificationType::DAOVote => 15,
            NotificationType::FundingUpdate => 16,
            NotificationType::PredictionResult => 17,
            NotificationType::SystemMessage => 18,
        }
    }
}

impl TryFrom<i32> for NotificationType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::InviteEditor),
            1 => Ok(Self::CancelEditor),
            2 => Ok(Self::AcceptEditor),
            3 => Ok(Self::DeclineEditor),
            4 => Ok(Self::NewBounty),
            5 => Ok(Self::NewDAO),
            6 => Ok(Self::NewPrediction),
            7 => Ok(Self::NewMessage),
            8 => Ok(Self::ApprovedBid),
            9 => Ok(Self::RejectedBid),
            10 => Ok(Self::BidReviewed),
            11 => Ok(Self::NewProject),
            12 => Ok(Self::ProjectMilestone),
            13 => Ok(Self::ProjectComment),
            14 => Ok(Self::BountyComment),
            15 => Ok(Self::DAOVote),
            16 => Ok(Self::FundingUpdate),
            17 => Ok(Self::PredictionResult),
            18 => Ok(Self::SystemMessage),
            _ => Err(format!("Invalid value for NotificationType: {}", value)),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Notification {
    pub id: i64,
    pub user_id: Uuid,
    pub notification_type: i32,
    pub title: String,
    pub message: String,
    pub data: Option<Value>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotification {
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationResponse {
    pub id: i64,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: Option<Value>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Notification> for NotificationResponse {
    fn from(notification: Notification) -> Self {
        Self {
            id: notification.id,
            user_id: notification.user_id,
            notification_type: NotificationType::try_from(notification.notification_type)
                .unwrap_or_default(),
            title: notification.title,
            message: notification.message,
            data: notification.data,
            is_read: notification.is_read,
            created_at: notification.created_at,
            updated_at: notification.updated_at,
        }
    }
}
