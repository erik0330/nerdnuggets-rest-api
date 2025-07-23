use crate::{pool::DatabasePool, repository::UserRepository, UtilRepository};
use std::{str::FromStr, sync::Arc};
use types::{
    dto::{
        UserAllSettingsResponse, UserCheckResponse, UserNotificationSettingsRequest,
        UserNotificationSettingsResponse, UserOnboardingRequest, UserPreferencesSettingsRequest,
        UserPreferencesSettingsResponse, UserPrivacySettingsRequest, UserPrivacySettingsResponse,
        UserProfileSettingsRequest, UserProfileSettingsResponse, UserWalletSettingsRequest,
        UserWalletSettingsResponse,
    },
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

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, ApiError> {
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

    pub async fn get_user_by_apple_id(&self, apple_id: &str) -> Result<User, ApiError> {
        self.user_repo
            .get_user_by_apple_id(apple_id)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn get_user_by_wallet(&self, wallet: &str) -> Result<User, ApiError> {
        self.user_repo
            .get_user_by_wallet(wallet)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn update_gmail(&self, id: Uuid, gmail: Option<String>) -> Result<bool, ApiError> {
        self.user_repo
            .update_gmail(id, gmail)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn update_apple_id(
        &self,
        id: Uuid,
        apple_id: Option<String>,
    ) -> Result<bool, ApiError> {
        self.user_repo
            .update_apple_id(id, apple_id)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
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
        let wallet_address = payload.wallet_address.filter(|w| !w.is_empty());
        self.user_repo
            .update_user_onboarding(
                id,
                &payload.name,
                &payload.institution,
                &payload.bio,
                payload.roles,
                payload.interests,
                wallet_address,
            )
            .await
            .map_err(|_| DbError::Str("Update user onboarding failed".to_string()).into())
    }

    pub async fn create_user_with_google(&self, gmail: &str, name: &str) -> Result<User, ApiError> {
        self.user_repo
            .create_user_with_google(gmail, name)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn create_user_with_apple(
        &self,
        apple_id: &str,
        email: Option<String>,
        name: Option<String>,
    ) -> Result<User, ApiError> {
        self.user_repo
            .create_user_with_apple(apple_id, email, name)
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

    // ========================= USER SETTINGS SERVICE METHODS =========================

    pub async fn get_all_user_settings(
        &self,
        user_id: Uuid,
    ) -> Result<UserAllSettingsResponse, ApiError> {
        let user = self.get_user_by_id(user_id).await?;

        Ok(UserAllSettingsResponse {
            profile: UserProfileSettingsResponse {
                id: user.id,
                avatar_url: user.avatar_url,
                email: user.email,
                name: user.name,
                institution: user.institution,
                bio: user.bio,
                website: user.website,
                roles: user.roles,
            },
            notifications: UserNotificationSettingsResponse {
                email_notifications: user.email_notifications,
                push_notifications: user.push_notifications,
                milestone_updates: user.milestone_updates,
                funding_updates: user.funding_updates,
                dao_proposals: user.dao_proposals,
                prediction_markets: user.prediction_markets,
            },
            privacy: UserPrivacySettingsResponse {
                profile_visibility: user.profile_visibility,
                show_funding_history: user.show_funding_history,
                show_prediction_history: user.show_prediction_history,
                two_factor_enabled: user.two_factor_enabled,
            },
            wallet: UserWalletSettingsResponse {
                wallet_address: user.wallet_address,
            },
            preferences: UserPreferencesSettingsResponse {
                dark_mode: user.dark_mode,
                language: user.language,
                timezone: user.timezone,
                display_currency: user.display_currency,
            },
        })
    }

    pub async fn update_profile_settings(
        &self,
        user_id: Uuid,
        payload: UserProfileSettingsRequest,
    ) -> Result<UserProfileSettingsResponse, ApiError> {
        let user = self
            .user_repo
            .update_profile_settings(
                user_id,
                payload.avatar_url,
                payload.email,
                payload.name,
                payload.institution,
                payload.bio,
                payload.website,
                payload.roles,
            )
            .await
            .map_err(|_| DbError::Str("Failed to update profile settings".to_string()))?;

        Ok(UserProfileSettingsResponse {
            id: user.id,
            avatar_url: user.avatar_url,
            email: user.email,
            name: user.name,
            institution: user.institution,
            bio: user.bio,
            website: user.website,
            roles: user.roles,
        })
    }

    pub async fn update_notification_settings(
        &self,
        user_id: Uuid,
        payload: UserNotificationSettingsRequest,
    ) -> Result<UserNotificationSettingsResponse, ApiError> {
        let user = self
            .user_repo
            .update_notification_settings(
                user_id,
                payload.email_notifications,
                payload.push_notifications,
                payload.milestone_updates,
                payload.funding_updates,
                payload.dao_proposals,
                payload.prediction_markets,
            )
            .await
            .map_err(|_| DbError::Str("Failed to update notification settings".to_string()))?;

        Ok(UserNotificationSettingsResponse {
            email_notifications: user.email_notifications,
            push_notifications: user.push_notifications,
            milestone_updates: user.milestone_updates,
            funding_updates: user.funding_updates,
            dao_proposals: user.dao_proposals,
            prediction_markets: user.prediction_markets,
        })
    }

    pub async fn update_privacy_settings(
        &self,
        user_id: Uuid,
        payload: UserPrivacySettingsRequest,
    ) -> Result<UserPrivacySettingsResponse, ApiError> {
        let user = self
            .user_repo
            .update_privacy_settings(
                user_id,
                payload.profile_visibility,
                payload.show_funding_history,
                payload.show_prediction_history,
                payload.two_factor_enabled,
            )
            .await
            .map_err(|_| DbError::Str("Failed to update privacy settings".to_string()))?;

        Ok(UserPrivacySettingsResponse {
            profile_visibility: user.profile_visibility,
            show_funding_history: user.show_funding_history,
            show_prediction_history: user.show_prediction_history,
            two_factor_enabled: user.two_factor_enabled,
        })
    }

    pub async fn update_wallet_settings(
        &self,
        user_id: Uuid,
        payload: UserWalletSettingsRequest,
    ) -> Result<UserWalletSettingsResponse, ApiError> {
        let user = self
            .user_repo
            .update_wallet_settings(user_id, payload.wallet_address.filter(|w| !w.is_empty()))
            .await
            .map_err(|_| DbError::Str("Failed to update wallet settings".to_string()))?;

        Ok(UserWalletSettingsResponse {
            wallet_address: user.wallet_address,
        })
    }

    pub async fn update_preferences_settings(
        &self,
        user_id: Uuid,
        payload: UserPreferencesSettingsRequest,
    ) -> Result<UserPreferencesSettingsResponse, ApiError> {
        let user = self
            .user_repo
            .update_preferences_settings(
                user_id,
                payload.dark_mode,
                payload.language,
                payload.timezone,
                payload.display_currency,
            )
            .await
            .map_err(|_| DbError::Str("Failed to update preferences settings".to_string()))?;

        Ok(UserPreferencesSettingsResponse {
            dark_mode: user.dark_mode,
            language: user.language,
            timezone: user.timezone,
            display_currency: user.display_currency,
        })
    }
}
