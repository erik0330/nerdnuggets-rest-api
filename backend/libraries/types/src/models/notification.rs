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
    InviteEditor,
    CancelEditor,
    AcceptEditor,
    DeclineEditor,
}

impl From<NotificationType> for i32 {
    fn from(notification_type: NotificationType) -> i32 {
        match notification_type {
            NotificationType::InviteEditor => 0,
            NotificationType::CancelEditor => 1,
            NotificationType::AcceptEditor => 2,
            NotificationType::DeclineEditor => 3,
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
