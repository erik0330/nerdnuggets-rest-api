use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Degree {
    pub id: Uuid,
    pub degree_name: String,
    pub abbreviation: Option<String>,
    pub is_available: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
