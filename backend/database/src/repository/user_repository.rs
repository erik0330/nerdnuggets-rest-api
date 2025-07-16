use crate::pool::DatabasePool;
use chrono::{DateTime, Utc};
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::{
    models::{ActivityHistory, TempUser, User},
    UserRoleType, UserTierType,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl UserRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_user_by_gmail(&self, gmail: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE gmail = $1")
            .bind(gmail)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_user_by_wallet(&self, wallet: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE wallet_address ILIKE $1")
            .bind(wallet)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_website(&self, web_site: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE web_site = $1")
            .bind(web_site)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_linkedin(&self, linkedin: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE linkedin = $1")
            .bind(linkedin)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_orc_id(&self, orc_id: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE orc_id = $1")
            .bind(orc_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn create_user_with_email(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, password, tier, verified_email)
            VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(name)
        .bind(email)
        .bind(password)
        .bind(UserTierType::Bronze.to_string())
        .bind(false)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        return Ok(user);
    }

    pub async fn create_user_with_google(&self, gmail: &str) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, verified_email, gmail)
            VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(gmail)
        .bind(true)
        .bind(gmail)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        return Ok(user);
    }

    pub async fn update_gmail(&self, id: Uuid, gmail: Option<String>) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE users SET gmail = $1 WHERE id = $2")
            .bind(gmail)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_editors(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<User>, SqlxError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE $1 = ANY(roles) LIMIT $2 OFFSET $3",
        )
        .bind(UserRoleType::Editor.to_string())
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(users)
    }

    pub async fn update_user_onboarding(
        &self,
        id: Uuid,
        name: &str,
        institution: &str,
        bio: &str,
        roles: Vec<String>,
        interests: Vec<String>,
        wallet_address: Option<String>,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>("UPDATE users SET name = $1, institution = $2, bio = $3, roles = $4, interests = $5, wallet_address = $6, updated_at = $7 WHERE id = $8 RETURNING *")
            .bind(name)
            .bind(institution)
            .bind(bio)
            .bind(roles)
            .bind(interests)
            .bind(wallet_address)
            .bind(Utc::now())
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(user)
    }

    pub async fn update_username(&self, id: Uuid, username: &str) -> Result<bool, SqlxError> {
        let update = sqlx::query("UPDATE users SET username = $1, updated_at = $2 WHERE id = $3")
            .bind(username)
            .bind(Utc::now())
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        return Ok(update.rows_affected() >= 1);
    }

    pub async fn create_activity(
        &self,
        user_id: Uuid,
        activity_type: String,
        description: String,
        details: Option<String>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO activity_history (user_id, activity_type, description, details) VALUES ($1, $2, $3, $4)")
            .bind(user_id)
            .bind(activity_type)
            .bind(description)
            .bind(details)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_activities(
        &self,
        user_id: Option<Uuid>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<ActivityHistory>, SqlxError> {
        let histories = if let Some(user_id) = user_id {
            sqlx::query_as::<_, ActivityHistory>(
                "SELECT * FROM activity_history WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(user_id)
            .bind(limit.unwrap_or(3))
            .bind(offset.unwrap_or(0))
            .fetch_all(self.db_conn.get_pool())
            .await?
        } else {
            sqlx::query_as::<_, ActivityHistory>(
                "SELECT * FROM activity_history LIMIT $1 OFFSET $2",
            )
            .bind(limit.unwrap_or(3))
            .bind(offset.unwrap_or(0))
            .fetch_all(self.db_conn.get_pool())
            .await?
        };
        Ok(histories)
    }

    // Temp user methods for email verification
    pub async fn tempuser_by_email(&self, email: &str) -> Result<TempUser, SqlxError> {
        let temp_user = sqlx::query_as::<_, TempUser>("SELECT * FROM temp_users WHERE email = $1")
            .bind(email)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(temp_user)
    }

    pub async fn create_tempuser_with_email(
        &self,
        email: &str,
        name: &str,
        password: &str,
        verify_type: &str,
        passkey: &str,
        try_limit: i16,
        iat: i64,
        exp: i64,
        now: DateTime<Utc>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query(
            "INSERT INTO temp_users (email, name, password, verify_type, passkey, try_limit, iat, exp, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(email)
        .bind(name)
        .bind(password)
        .bind(verify_type)
        .bind(passkey)
        .bind(try_limit)
        .bind(iat)
        .bind(exp)
        .bind(now)
        .bind(now)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_tempuser_with_email(
        &self,
        email: &str,
        name: &str,
        password: &str,
        verify_type: &str,
        passkey: &str,
        try_limit: i16,
        iat: i64,
        exp: i64,
        now: chrono::DateTime<Utc>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query(
            "UPDATE temp_users SET name = $1, password = $2, verify_type = $3, passkey = $4, try_limit = $5, iat = $6, exp = $7, updated_at = $8 WHERE email = $9",
        )
        .bind(name)
        .bind(password)
        .bind(verify_type)
        .bind(passkey)
        .bind(try_limit)
        .bind(iat)
        .bind(exp)
        .bind(now)
        .bind(email)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn delete_tempuser_by_email(&self, email: &str) -> Result<bool, SqlxError> {
        let row = sqlx::query("DELETE FROM temp_users WHERE email = $1")
            .bind(email)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn verify_user_email(&self, email: &str) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE users SET verified_email = true WHERE email = $1")
            .bind(email)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    // ========================= USER SETTINGS METHODS =========================

    pub async fn update_profile_settings(
        &self,
        id: Uuid,
        avatar_url: Option<String>,
        email: String,
        name: Option<String>,
        institution: Option<String>,
        bio: Option<String>,
        website: Option<String>,
        roles: Vec<String>,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET avatar_url = $1, email = $2, name = $3, institution = $4, bio = $5, website = $6, roles = $7, updated_at = $8 WHERE id = $9 RETURNING *"
        )
        .bind(avatar_url)
        .bind(email)
        .bind(name)
        .bind(institution)
        .bind(bio)
        .bind(website)
        .bind(roles)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(user)
    }

    pub async fn update_notification_settings(
        &self,
        id: Uuid,
        email_notifications: bool,
        push_notifications: bool,
        milestone_updates: bool,
        funding_updates: bool,
        dao_proposals: bool,
        prediction_markets: bool,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET email_notifications = $1, push_notifications = $2, milestone_updates = $3, funding_updates = $4, dao_proposals = $5, prediction_markets = $6, updated_at = $7 WHERE id = $8 RETURNING *"
        )
        .bind(email_notifications)
        .bind(push_notifications)
        .bind(milestone_updates)
        .bind(funding_updates)
        .bind(dao_proposals)
        .bind(prediction_markets)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(user)
    }

    pub async fn update_privacy_settings(
        &self,
        id: Uuid,
        profile_visibility: bool,
        show_funding_history: bool,
        show_prediction_history: bool,
        two_factor_enabled: bool,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET profile_visibility = $1, show_funding_history = $2, show_prediction_history = $3, two_factor_enabled = $4, updated_at = $5 WHERE id = $6 RETURNING *"
        )
        .bind(profile_visibility)
        .bind(show_funding_history)
        .bind(show_prediction_history)
        .bind(two_factor_enabled)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(user)
    }

    pub async fn update_wallet_settings(
        &self,
        id: Uuid,
        wallet_address: Option<String>,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET wallet_address = $1, updated_at = $2 WHERE id = $3 RETURNING *",
        )
        .bind(wallet_address)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(user)
    }

    pub async fn update_preferences_settings(
        &self,
        id: Uuid,
        dark_mode: bool,
        language: String,
        timezone: String,
        display_currency: String,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET dark_mode = $1, language = $2, timezone = $3, display_currency = $4, updated_at = $5 WHERE id = $6 RETURNING *"
        )
        .bind(dark_mode)
        .bind(language)
        .bind(timezone)
        .bind(display_currency)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(user)
    }
}
