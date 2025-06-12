use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Affiliation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub institution: Option<String>,
    pub department: Option<String>,
    pub is_current: Option<bool>,
    pub institution_address: Option<String>,
    pub line_2: Option<String>,
    pub line_3: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub work_phone_number: Option<String>,
    //will remove from
    pub domain_expertise: Option<Vec<String>>,
    pub years: Option<i32>,
    pub numbers: Option<i32>,
    pub journals: Option<Vec<String>>,
    pub publications: Option<Vec<String>>,
    //will remove to
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
