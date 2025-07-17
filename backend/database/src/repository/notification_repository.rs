use std::sync::Arc;

use crate::DatabasePool;
use sqlx::Error as SqlxError;
use types::models::{CreateNotification, Notification, NotificationCount, NotificationTab};
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

    /// Creates a new notification in the database
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

    /// Gets all notifications for a user with pagination
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

    /// Gets notifications for a user filtered by tab with pagination
    /// Supports: All, Unread, Funding, DAO, Projects, Predictions, Site
    pub async fn get_user_notifications_by_tab(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
        tab: Option<NotificationTab>,
    ) -> Result<Vec<Notification>, SqlxError> {
        let tab_enum = tab.unwrap_or(NotificationTab::All);
        let notification_types = tab_enum.to_notification_types();

        let notifications = match tab_enum {
            NotificationTab::All => {
                // All notifications, no filtering
                sqlx::query_as::<_, Notification>(
                    "
                    SELECT id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
                    FROM notifications
                    WHERE user_id = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "
                )
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.db_conn.get_pool())
                .await?
            }
            NotificationTab::Unread => {
                // Only unread notifications
                sqlx::query_as::<_, Notification>(
                    "
                    SELECT id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
                    FROM notifications
                    WHERE user_id = $1 AND is_read = false
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "
                )
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.db_conn.get_pool())
                .await?
            }
            _ => {
                if notification_types.is_empty() {
                    // No specific types to filter, return all
                    sqlx::query_as::<_, Notification>(
                        "
                        SELECT id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
                        FROM notifications
                        WHERE user_id = $1
                        ORDER BY created_at DESC
                        LIMIT $2 OFFSET $3
                        "
                    )
                    .bind(user_id)
                    .bind(limit)
                    .bind(offset)
                    .fetch_all(self.db_conn.get_pool())
                    .await?
                } else {
                    // Build the SQL for filtering by notification_type
                    // We use a dynamic number of $N parameters for the IN clause
                    let type_count = notification_types.len();
                    let placeholders: Vec<String> =
                        (0..type_count).map(|i| format!("${}", i + 2)).collect();

                    let sql = format!(
                        "SELECT id, user_id, notification_type, title, message, data, is_read, created_at, updated_at
                         FROM notifications
                         WHERE user_id = $1 AND notification_type IN ({})
                         ORDER BY created_at DESC
                         LIMIT ${} OFFSET ${}",
                        placeholders.join(", "),
                        type_count + 2,
                        type_count + 3
                    );

                    let mut query = sqlx::query_as::<_, Notification>(&sql).bind(user_id);

                    // Bind notification type parameters
                    for notification_type in &notification_types {
                        let type_value: i32 = notification_type.clone().into();
                        query = query.bind(type_value);
                    }

                    // Bind limit and offset
                    query = query.bind(limit).bind(offset);

                    query.fetch_all(self.db_conn.get_pool()).await?
                }
            }
        };

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

    /// Gets the total count of notifications for a user
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

    /// Gets the count of notifications for a user filtered by tab
    /// Supports: All, Unread, Funding, DAO, Projects, Predictions, Site
    pub async fn get_notification_count_by_tab(
        &self,
        user_id: Uuid,
        tab: Option<NotificationTab>,
    ) -> Result<i64, SqlxError> {
        let tab_enum = tab.unwrap_or(NotificationTab::All);
        let notification_types = tab_enum.to_notification_types();

        let count = match tab_enum {
            NotificationTab::All => sqlx::query!(
                r#"
                    SELECT COUNT(*) as count
                    FROM notifications
                    WHERE user_id = $1
                    "#,
                user_id
            )
            .fetch_one(self.db_conn.get_pool())
            .await?
            .count
            .unwrap_or(0),
            NotificationTab::Unread => sqlx::query!(
                r#"
                    SELECT COUNT(*) as count
                    FROM notifications
                    WHERE user_id = $1 AND is_read = false
                    "#,
                user_id
            )
            .fetch_one(self.db_conn.get_pool())
            .await?
            .count
            .unwrap_or(0),
            _ => {
                if notification_types.is_empty() {
                    // No specific types to filter, count all
                    sqlx::query!(
                        r#"
                        SELECT COUNT(*) as count
                        FROM notifications
                        WHERE user_id = $1
                        "#,
                        user_id
                    )
                    .fetch_one(self.db_conn.get_pool())
                    .await?
                    .count
                    .unwrap_or(0)
                } else {
                    // Build the SQL for filtering by notification_type
                    let type_count = notification_types.len();
                    let placeholders: Vec<String> =
                        (0..type_count).map(|i| format!("${}", i + 2)).collect();

                    let sql = format!(
                        "SELECT COUNT(*) as count FROM notifications WHERE user_id = $1 AND notification_type IN ({})",
                        placeholders.join(", ")
                    );

                    let mut query = sqlx::query_as::<_, NotificationCount>(&sql).bind(user_id);

                    // Bind notification type parameters
                    for notification_type in &notification_types {
                        let type_value: i32 = notification_type.clone().into();
                        query = query.bind(type_value);
                    }

                    query.fetch_one(self.db_conn.get_pool()).await?.count
                }
            }
        };

        Ok(count)
    }

    pub async fn get_unread_count_by_tab(
        &self,
        user_id: Uuid,
        tab: Option<NotificationTab>,
    ) -> Result<i64, SqlxError> {
        let tab_enum = tab.unwrap_or(NotificationTab::All);
        let notification_types = tab_enum.to_notification_types();

        let count = match tab_enum {
            NotificationTab::All => sqlx::query!(
                r#"
                    SELECT COUNT(*) as count
                    FROM notifications
                    WHERE user_id = $1
                    AND is_read = false
                    "#,
                user_id
            )
            .fetch_one(self.db_conn.get_pool())
            .await?
            .count
            .unwrap_or(0),
            NotificationTab::Unread => sqlx::query!(
                r#"
                    SELECT COUNT(*) as count
                    FROM notifications
                    WHERE user_id = $1
                    AND is_read = false
                    "#,
                user_id
            )
            .fetch_one(self.db_conn.get_pool())
            .await?
            .count
            .unwrap_or(0),
            _ => {
                if notification_types.is_empty() {
                    // No specific types to filter, count all
                    sqlx::query!(
                        r#"
                        SELECT COUNT(*) as count
                        FROM notifications
                        WHERE user_id = $1
                        AND is_read = false
                        "#,
                        user_id
                    )
                    .fetch_one(self.db_conn.get_pool())
                    .await?
                    .count
                    .unwrap_or(0)
                } else {
                    // Build the SQL for filtering by notification_type
                    let type_count = notification_types.len();
                    let placeholders: Vec<String> =
                        (0..type_count).map(|i| format!("${}", i + 2)).collect();

                    let sql = format!(
                        "SELECT COUNT(*) as count FROM notifications WHERE user_id = $1 AND notification_type IN ({}) AND is_read = false",
                        placeholders.join(", ")
                    );

                    let mut query = sqlx::query_as::<_, NotificationCount>(&sql).bind(user_id);

                    // Bind notification type parameters
                    for notification_type in &notification_types {
                        let type_value: i32 = notification_type.clone().into();
                        query = query.bind(type_value);
                    }

                    query.fetch_one(self.db_conn.get_pool()).await?.count
                }
            }
        };

        Ok(count)
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
