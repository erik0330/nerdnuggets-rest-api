use std::sync::Arc;

use crate::DatabasePool;
use sqlx::Error as SqlxError;
use types::models::{CreateNotification, Notification};
use uuid::Uuid;

#[derive(Clone)]
pub struct NotificationRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl NotificationRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_notification(
        &self,
        notification: CreateNotification,
    ) -> Result<Notification, SqlxError> {
        let notification_type: i32 = notification.notification_type.into();
        let notification = sqlx::query_as::<_, Notification>(
            "
            INSERT INTO notifications (user_id, notification_type, title, message, data)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
            ",
        )
        .bind(notification.user_id)
        .bind(notification_type)
        .bind(notification.title)
        .bind(notification.message)
        .bind(notification.data)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(notification)
    }

    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, SqlxError> {
        let notifications = sqlx::query_as::<_, Notification>(
            "
            SELECT id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            ",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(notifications)
    }

    pub async fn get_unread_notifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Notification>, SqlxError> {
        let notifications = sqlx::query_as::<_, Notification>(
            "
            SELECT id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
            FROM notifications
            WHERE user_id = $1 AND is_read = false
            ORDER BY created_at DESC
            ",
        )
        .bind(user_id)
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(notifications)
    }

    pub async fn mark_notification_as_read(
        &self,
        notification_id: i64,
        user_id: Uuid,
    ) -> Result<(), SqlxError> {
        sqlx::query(
            "
            UPDATE notifications
            SET is_read = true
            WHERE id = $1 AND user_id = $2
            ",
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(self.db_conn.get_pool())
        .await?;

        Ok(())
    }

    pub async fn mark_all_notifications_as_read(&self, user_id: Uuid) -> Result<(), SqlxError> {
        sqlx::query(
            "
            UPDATE notifications
            SET is_read = true
            WHERE user_id = $1
            ",
        )
        .bind(user_id)
        .execute(self.db_conn.get_pool())
        .await?;

        Ok(())
    }

    pub async fn delete_notification(
        &self,
        notification_id: i64,
        user_id: Uuid,
    ) -> Result<(), SqlxError> {
        sqlx::query(
            "
            DELETE FROM notifications
            WHERE id = $1 AND user_id = $2
            ",
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(self.db_conn.get_pool())
        .await?;

        Ok(())
    }

    pub async fn get_notification_count(&self, user_id: Uuid) -> Result<i64, SqlxError> {
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM notifications
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(row.count.unwrap_or(0))
    }

    pub async fn get_unread_count(&self, user_id: Uuid) -> Result<i64, SqlxError> {
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM notifications
            WHERE user_id = $1 AND is_read = false
            "#,
            user_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(row.count.unwrap_or(0))
    }

    pub async fn get_latest_notification_index(&self) -> Result<i64, SqlxError> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(MAX(id), 0) as max_id
            FROM notifications
            "#
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(row.max_id.unwrap_or(0))
    }

    pub async fn get_notifications_from_index(
        &self,
        from_index: i64,
        limit: i64,
    ) -> Result<Vec<Notification>, SqlxError> {
        let notifications = sqlx::query_as::<_, Notification>(
            "
            SELECT id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
            FROM notifications
            WHERE id > $1
            ORDER BY id ASC
            LIMIT $2
            ",
        )
        .bind(from_index)
        .bind(limit)
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(notifications)
    }
}
