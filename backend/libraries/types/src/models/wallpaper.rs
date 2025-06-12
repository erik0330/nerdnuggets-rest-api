use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct WallPapers {
    pub id: Uuid,
    pub wallpaper: String,
    pub created_at: Option<DateTime<Utc>>,
}
