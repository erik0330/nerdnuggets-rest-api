use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: String,
    pub verified_email: bool,
    pub gmail: Option<String>,
    pub roles: Vec<String>,
    pub institution: Option<String>,
    pub interests: Vec<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    // wallet
    pub tier: String,
    pub nerd_balance: i64,
    pub wallet_address: Option<String>,
    // date
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn to_info(&self) -> UserInfo {
        UserInfo {
            id: self.id,
            username: self.username.clone().unwrap_or_default(),
            name: self.name.clone().unwrap_or_default(),
            email: self.email.clone(),
            roles: self.roles.clone(),
            institution: self.institution.clone().unwrap_or_default(),
            interests: self.interests.clone(),
            avatar_url: self.avatar_url.clone(),
            wallet_address: self.wallet_address.clone(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub roles: Vec<String>,
    pub institution: String,
    pub interests: Vec<String>,
    pub avatar_url: Option<String>,
    pub wallet_address: Option<String>,
}
