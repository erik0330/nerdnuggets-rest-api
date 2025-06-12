use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserInfo;

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
    FollowUser,
    ResearchCheckSuccess,
    ResearchCheckFailed,
    PostLike,
    CommentLike,
    PostComment,
    CommentReply,
    ApproveAdminRole,
    ApproveEditorRole,
    ApproveReviewerRole,
    ApproveAuthorRole,
    ApproveCopyEditorRole,
    //article
    InviteEditor,
    CancelEditor,
    AcceptEditor,
    DeclineEditor,
    InviteReviewer,
    CancelReviewer,
    AcceptReviewer,
    DeclineReviewer,
    NerdBunnyResearchCheckSuccess,
    NerdBunnyResearchCheckFailed,
    InviteCopyEditor,
    CancelCopyEditor,
    AcceptCopyEditor,
    DeclineCopyEditor,
    AIErrorCheckSuccess,
    AIErrorCheckFailed,
}

impl From<NotificationType> for i32 {
    fn from(notification_type: NotificationType) -> i32 {
        match notification_type {
            NotificationType::FollowUser => 0,
            NotificationType::ResearchCheckSuccess => 1,
            NotificationType::ResearchCheckFailed => 2,
            NotificationType::PostLike => 3,
            NotificationType::CommentLike => 4,
            NotificationType::PostComment => 5,
            NotificationType::CommentReply => 6,
            NotificationType::ApproveAdminRole => 7,
            NotificationType::ApproveEditorRole => 8,
            NotificationType::ApproveReviewerRole => 9,
            NotificationType::ApproveAuthorRole => 10,
            NotificationType::InviteEditor => 11,
            NotificationType::CancelEditor => 12,
            NotificationType::AcceptEditor => 13,
            NotificationType::DeclineEditor => 14,
            NotificationType::InviteReviewer => 15,
            NotificationType::CancelReviewer => 16,
            NotificationType::AcceptReviewer => 17,
            NotificationType::DeclineReviewer => 18,
            NotificationType::NerdBunnyResearchCheckSuccess => 19,
            NotificationType::NerdBunnyResearchCheckFailed => 20,
            NotificationType::InviteCopyEditor => 21,
            NotificationType::CancelCopyEditor => 22,
            NotificationType::AcceptCopyEditor => 23,
            NotificationType::DeclineCopyEditor => 24,
            NotificationType::ApproveCopyEditorRole => 25,
            NotificationType::AIErrorCheckSuccess => 26,
            NotificationType::AIErrorCheckFailed => 27,
        }
    }
}

impl TryFrom<i32> for NotificationType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FollowUser),
            1 => Ok(Self::ResearchCheckSuccess),
            2 => Ok(Self::ResearchCheckFailed),
            3 => Ok(Self::PostLike),
            4 => Ok(Self::CommentLike),
            5 => Ok(Self::PostComment),
            6 => Ok(Self::CommentReply),
            7 => Ok(Self::ApproveAdminRole),
            8 => Ok(Self::ApproveEditorRole),
            9 => Ok(Self::ApproveReviewerRole),
            10 => Ok(Self::ApproveAuthorRole),
            11 => Ok(Self::InviteEditor),
            12 => Ok(Self::CancelEditor),
            13 => Ok(Self::AcceptEditor),
            14 => Ok(Self::DeclineEditor),
            15 => Ok(Self::InviteReviewer),
            16 => Ok(Self::CancelReviewer),
            17 => Ok(Self::AcceptReviewer),
            18 => Ok(Self::DeclineReviewer),
            19 => Ok(Self::NerdBunnyResearchCheckSuccess),
            20 => Ok(Self::NerdBunnyResearchCheckFailed),
            21 => Ok(Self::InviteCopyEditor),
            22 => Ok(Self::CancelCopyEditor),
            23 => Ok(Self::AcceptCopyEditor),
            24 => Ok(Self::DeclineCopyEditor),
            25 => Ok(Self::ApproveCopyEditorRole),
            26 => Ok(Self::AIErrorCheckSuccess),
            27 => Ok(Self::AIErrorCheckFailed),
            _ => Err(format!("Invalid value for NotificationType: {}", value)),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Notification {
    pub id: i64,
    pub message_type: i32,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub _type: i32,
    pub user_id: Uuid,
    pub referrer_id: Option<Uuid>,
    pub payload: Option<String>,
    pub read_status: bool,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn to_info(&self, referrer: Option<UserInfo>) -> NotificationInfo {
        NotificationInfo {
            id: self.id,
            _type: self._type.try_into().unwrap_or_default(),
            message_type: self.message_type.try_into().unwrap_or_default(),
            user_id: self.user_id.clone(),
            referrer,
            payload: self.payload.clone(),
            read_status: self.read_status,
            created_at: self.created_at.clone(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationInfo {
    pub id: i64,
    pub message_type: MessageType,
    #[serde(rename = "type")]
    pub _type: NotificationType,
    pub user_id: Uuid,
    pub referrer: Option<UserInfo>,
    pub payload: Option<String>,
    pub read_status: bool,
    pub created_at: DateTime<Utc>,
}
