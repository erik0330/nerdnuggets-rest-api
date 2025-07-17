use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use types::{
    error::{ApiError, UserError},
    models::{CreateNotification, NotificationResponse, NotificationType, User},
};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct NotificationCountResponse {
    pub total: i64,
    pub unread: i64,
}

pub async fn get_notifications(
    Extension(user): Extension<User>,
    Query(query): Query<NotificationQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<NotificationResponse>>, ApiError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    Ok(Json(
        state
            .service
            .notification
            .get_user_notifications(user.id, limit, offset)
            .await?,
    ))
}

pub async fn get_unread_notifications(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Vec<NotificationResponse>>, ApiError> {
    Ok(Json(
        state
            .service
            .notification
            .get_unread_notifications(user.id)
            .await?,
    ))
}

pub async fn mark_notification_as_read(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(notification_id): Path<i64>,
) -> Result<Json<()>, ApiError> {
    Ok(Json(
        state
            .service
            .notification
            .mark_notification_as_read(notification_id, user.id)
            .await?,
    ))
}

pub async fn mark_all_notifications_as_read(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<()>, ApiError> {
    Ok(Json(
        state
            .service
            .notification
            .mark_all_notifications_as_read(user.id)
            .await?,
    ))
}

pub async fn delete_notification(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(notification_id): Path<i64>,
) -> Result<Json<()>, ApiError> {
    Ok(Json(
        state
            .service
            .notification
            .delete_notification(notification_id, user.id)
            .await?,
    ))
}

pub async fn get_notification_count(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<NotificationCountResponse>, ApiError> {
    let total = state
        .service
        .notification
        .get_notification_count(user.id)
        .await?;

    let unread = state.service.notification.get_unread_count(user.id).await?;

    Ok(Json(NotificationCountResponse { total, unread }))
}

// Admin endpoints for creating notifications
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub async fn create_notification(
    Extension(_user): Extension<User>,
    Extension(role): Extension<String>,
    State(state): State<AppState>,
    Json(request): Json<CreateNotificationRequest>,
) -> Result<Json<NotificationResponse>, ApiError> {
    if role != "admin" {
        return Err(ApiError::UserError(UserError::Str(
            "You are not authorized to create notifications".to_string(),
        )));
    }

    let notification = CreateNotification {
        user_id: request.user_id,
        notification_type: request.notification_type,
        title: request.title,
        message: request.message,
        data: request.data,
    };

    Ok(Json(NotificationResponse::from(
        state
            .service
            .notification
            .create_notification(notification)
            .await?,
    )))
}
