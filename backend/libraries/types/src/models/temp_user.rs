use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct TempUser {
    pub id: Uuid,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub verify_type: Option<String>,
    pub passkey: Option<String>,
    pub try_limit: Option<i16>,
    pub iat: Option<i64>,
    pub exp: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
