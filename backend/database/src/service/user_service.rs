use crate::{pool::DatabasePool, repository::UserRepository, UtilRepository};
use std::{collections::HashSet, str::FromStr, sync::Arc};
use types::{
    dto::{
        UserAllSettingsResponse, UserCheckResponse, UserNotificationSettingsRequest,
        UserNotificationSettingsResponse, UserOnboardingRequest, UserPreferencesSettingsRequest,
        UserPreferencesSettingsResponse, UserPrivacySettingsRequest, UserPrivacySettingsResponse,
        UserProfileResponse, UserProfileSettingsRequest, UserProfileSettingsResponse,
        UserWalletSettingsRequest, UserWalletSettingsResponse,
    },
    error::{ApiError, DbError, UserError},
    models::{ActivityHistory, TempUser, User, UserInfo},
};
use utils::commons;
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

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, ApiError> {
        self.user_repo
            .get_user_by_username(username)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn get_all_usernames(&self) -> Result<Vec<String>, ApiError> {
        self.user_repo
            .get_all_usernames()
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
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
        name: Option<String>,
    ) -> Result<Vec<UserInfo>, ApiError> {
        let users = self
            .user_repo
            .get_editors(offset, limit, name)
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
        // Generate unique username
        let existing_usernames = self.get_all_usernames().await.unwrap_or_default();
        let existing_usernames_set: HashSet<String> = existing_usernames.into_iter().collect();
        let username = commons::generate_username(Some(name), gmail, &existing_usernames_set);

        self.user_repo
            .create_user_with_google_and_username(gmail, name, &username)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn create_user_with_apple(
        &self,
        apple_id: &str,
        email: Option<String>,
        name: Option<String>,
    ) -> Result<User, ApiError> {
        // Generate unique username
        let existing_usernames = self.get_all_usernames().await.unwrap_or_default();
        let existing_usernames_set: HashSet<String> = existing_usernames.into_iter().collect();
        let email_str = email.as_deref().unwrap_or("");
        let username =
            commons::generate_username(name.as_deref(), email_str, &existing_usernames_set);

        self.user_repo
            .create_user_with_apple_and_username(apple_id, email, name, &username)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn check_email(&self, email: &str) -> Result<UserCheckResponse, ApiError> {
        Ok(UserCheckResponse {
            is_available: self.user_repo.get_user_by_email(email).await.is_none(),
        })
    }

    pub async fn check_username(&self, username: &str) -> Result<UserCheckResponse, ApiError> {
        Ok(UserCheckResponse {
            is_available: self
                .user_repo
                .get_user_by_username(username)
                .await
                .is_none(),
        })
    }

    pub async fn update_username(&self, user_id: Uuid, username: &str) -> Result<bool, ApiError> {
        // Check if username is already taken by another user
        let existing_user = self.user_repo.get_user_by_username(username).await;
        if let Some(existing_user) = existing_user {
            if existing_user.id != user_id {
                return Err(UserError::UsernameAlreadyExists)?;
            }
        }

        self.user_repo
            .update_username(user_id, username)
            .await
            .map_err(|_| DbError::Str("Failed to update username".to_string()).into())
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

        // Generate unique username
        let existing_usernames = self.get_all_usernames().await.unwrap_or_default();
        let existing_usernames_set: HashSet<String> = existing_usernames.into_iter().collect();
        let username = commons::generate_username(Some(name), email, &existing_usernames_set);

        match self
            .user_repo
            .create_user_with_email_and_username(name, email, password, &username)
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

    pub async fn update_password(&self, user_id: Uuid, password: &str) -> Result<bool, ApiError> {
        self.user_repo
            .update_password(user_id, password)
            .await
            .map_err(|_| DbError::Str("Failed to update password".to_string()).into())
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
                username: user.username.unwrap_or_default(),
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
        // Handle username update if provided
        if let Some(username) = &payload.username.filter(|u| !u.is_empty()) {
            self.update_username(user_id, username).await?;
        }

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
            username: user.username.unwrap_or_default(),
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

    pub async fn get_user_profile_by_username(
        &self,
        username: &str,
    ) -> Result<UserProfileResponse, ApiError> {
        let user = self.get_user_by_username(username).await?;

        // Get counts
        let projects_count = self
            .user_repo
            .count_user_projects(user.id)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        let bounties_count = self
            .user_repo
            .count_user_bounties(user.id)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        let predictions_count = self
            .user_repo
            .count_user_predictions(user.id)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        let contributions_count = self
            .user_repo
            .count_user_contributions(user.id)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        Ok(UserProfileResponse {
            id: user.id,
            username: user.username.unwrap_or_default(),
            name: user.name.unwrap_or_default(),
            email: user.email,
            roles: user.roles,
            institution: user.institution.unwrap_or_default(),
            interests: user.interests,
            avatar_url: user.avatar_url,
            bio: user.bio,
            website: user.website,
            tier: user.tier,
            nerd_balance: user.nerd_balance,
            wallet_address: user.wallet_address,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
            projects_count,
            bounties_count,
            predictions_count,
            contributions_count,
        })
    }
}
