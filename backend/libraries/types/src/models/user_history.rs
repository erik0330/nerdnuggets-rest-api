use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct UserHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub liked_posts: Option<Vec<Uuid>>,
    pub liked_post_comments: Option<Vec<Uuid>>,
    pub bookmark_posts: Option<Vec<Uuid>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
