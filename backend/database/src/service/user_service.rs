use crate::{pool::DatabasePool, repository::UserRepository, UtilRepository};
use std::{str::FromStr, sync::Arc};
use types::{
    dto::{UserCheckResponse, UserOnboardingRequest},
    error::{ApiError, DbError, UserError},
    models::{ActivityHistory, TempUser, User, UserInfo},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    _util_repo: UtilRepository,
}

impl UserService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            _util_repo: UtilRepository::new(db_conn),
        }
    }

    pub async fn get_by_user_id(&self, user_id: Uuid) -> Result<User, ApiError> {
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, ApiError> {
        self.user_repo
            .get_user_by_email(email)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn get_user_by_gmail(&self, gmail: &str) -> Result<User, ApiError> {
        self.user_repo
            .get_user_by_gmail(gmail)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn update_gmail(&self, id: Uuid, gmail: Option<String>) -> Result<bool, ApiError> {
        self.user_repo
            .update_gmail(id, gmail)
            .await
            .map_err(|_| DbError::Str("Update gmail failed".to_string()).into())
    }

    pub async fn get_editors(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<UserInfo>, ApiError> {
        let users = self
            .user_repo
            .get_editors(offset, limit)
            .await
            .map_err(|_| DbError::Str("Get editors failed".to_string()))?;
        Ok(users.iter().map(|u| u.to_info()).collect())
    }

    pub async fn update_user_onboarding(
        &self,
        id: &str,
        payload: UserOnboardingRequest,
    ) -> Result<User, ApiError> {
        let id = Uuid::from_str(id)
            .map_err(|_| ApiError::DbError(DbError::Str("Invalid UUID format".to_string())))?;
        self.user_repo
            .update_user_onboarding(
                id,
                &payload.name,
                &payload.institution,
                &payload.bio,
                payload.roles,
                payload.interests,
                payload.wallet_address,
            )
            .await
            .map_err(|_| DbError::Str("Update user onboarding failed".to_string()).into())
    }

    pub async fn create_user_with_google(&self, gmail: &str) -> Result<User, ApiError> {
        self.user_repo
            .create_user_with_google(gmail)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn check_email(&self, email: &str) -> Result<UserCheckResponse, ApiError> {
        Ok(UserCheckResponse {
            is_available: self.user_repo.get_user_by_email(email).await.is_none(),
        })
    }

    pub fn verify_password(&self, user: &User, password: &str) -> bool {
        bcrypt::verify(password, user.password.clone().unwrap().as_str()).unwrap_or(false)
    }

    pub async fn create_user_with_email(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<User, ApiError> {
        if self.user_repo.get_user_by_email(email).await.is_some() {
            return Err(UserError::UserAlreadyExists)?;
        }
        match self
            .user_repo
            .create_user_with_email(name, email, password)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => Err(DbError::Str(e.to_string()))?,
        }
    }

    pub async fn get_activities(
        &self,
        user_id: Option<Uuid>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<ActivityHistory>, ApiError> {
        let histories = self
            .user_repo
            .get_activities(user_id, offset, limit)
            .await
            .unwrap_or_default();
        Ok(histories)
    }

    // Temp user methods for email verification
    pub async fn tempuser_by_email(&self, email: &str) -> Result<TempUser, ApiError> {
        self.user_repo
            .tempuser_by_email(email)
            .await
            .map_err(|_| UserError::TempUserNotFound.into())
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
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<bool, ApiError> {
        self.user_repo
            .create_tempuser_with_email(
                email,
                name,
                password,
                verify_type,
                passkey,
                try_limit,
                iat,
                exp,
                now,
            )
            .await
            .map_err(|_| DbError::Str("Failed to create temp user".to_string()).into())
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
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<bool, ApiError> {
        self.user_repo
            .update_tempuser_with_email(
                email,
                name,
                password,
                verify_type,
                passkey,
                try_limit,
                iat,
                exp,
                now,
            )
            .await
            .map_err(|_| DbError::Str("Failed to update temp user".to_string()).into())
    }

    pub async fn delete_tempuser_by_email(&self, email: &str) -> Result<bool, ApiError> {
        self.user_repo
            .delete_tempuser_by_email(email)
            .await
            .map_err(|_| DbError::Str("Failed to delete temp user".to_string()).into())
    }

    pub async fn verify_user_email(&self, email: &str) -> Result<bool, ApiError> {
        self.user_repo
            .verify_user_email(email)
            .await
            .map_err(|_| DbError::Str("Failed to verify user email".to_string()).into())
    }
}
