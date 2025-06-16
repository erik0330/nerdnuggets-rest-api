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
    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id,
            username: self.username.clone().unwrap_or_default(),
            name: self.name.clone().unwrap_or_default(),
            password: self.password.clone(),
            email: self.email.clone(),
            verified_email: self.verified_email,
            gmail: self.gmail.clone(),
            roles: self.roles.clone(),
            avatar_url: self.avatar_url.clone(),
            bio: self.bio.clone(),
            tier: self.tier.clone(),
            nerd_balance: self.nerd_balance,
            wallet_address: self.wallet_address.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }

    pub fn get_profile_link(&self, frontend_url: &str) -> String {
        format!(
            "{}/@{}",
            frontend_url,
            self.username.clone().unwrap_or_default()
        )
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub password: Option<String>,
    pub email: String,
    pub verified_email: bool,
    pub gmail: Option<String>,
    pub roles: Vec<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    // wallet
    pub tier: String,
    pub nerd_balance: i64,
    pub wallet_address: Option<String>,
    // date
    pub created_at: String,
    pub updated_at: String,
}
