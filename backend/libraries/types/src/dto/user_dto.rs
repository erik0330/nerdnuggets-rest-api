use crate::models::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginWithEmailRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginWithGoogleRequest {
    pub access_token: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginWithAppleRequest {
    pub authorization_code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReadDto {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub roles: Vec<String>,
    pub institution: String,
    pub interests: Vec<String>,
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

impl UserReadDto {
    pub fn from(model: User) -> UserReadDto {
        Self {
            id: model.id,
            username: model.username.unwrap_or_default(),
            name: model.name.unwrap_or_default(),
            email: model.email,
            roles: model.roles,
            avatar_url: model.avatar_url,
            institution: model.institution.unwrap_or_default(),
            interests: model.interests,
            bio: model.bio,
            tier: model.tier,
            nerd_balance: model.nerd_balance,
            wallet_address: model.wallet_address,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginAndRegisterResponse {
    pub user: UserReadDto,
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct UserCheckEmailOption {
    pub email: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCheckResponse {
    pub is_available: bool,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterWithEmailRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetEditorsOption {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserOnboardingRequest {
    pub name: String,
    pub institution: String,
    pub bio: String,
    pub roles: Vec<String>,
    pub interests: Vec<String>,
    pub wallet_address: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResendVerificationEmailRequest {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailVerificationResponse {
    pub is_sent: bool,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEmailRequest {
    pub email: String,
    pub verification_code: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct ChangeRoleRequest {
    pub role: String,
}

// ========================= USER SETTINGS DTOs =========================

// Profile Settings
#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileSettingsRequest {
    pub avatar_url: Option<String>,
    pub email: String,
    pub name: Option<String>,
    pub institution: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileSettingsResponse {
    pub id: Uuid,
    pub avatar_url: Option<String>,
    pub email: String,
    pub name: Option<String>,
    pub institution: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub roles: Vec<String>,
}

// Notification Settings
#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserNotificationSettingsRequest {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub milestone_updates: bool,
    pub funding_updates: bool,
    pub dao_proposals: bool,
    pub prediction_markets: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserNotificationSettingsResponse {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub milestone_updates: bool,
    pub funding_updates: bool,
    pub dao_proposals: bool,
    pub prediction_markets: bool,
}

// Privacy Settings
#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPrivacySettingsRequest {
    pub profile_visibility: bool,
    pub show_funding_history: bool,
    pub show_prediction_history: bool,
    pub two_factor_enabled: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPrivacySettingsResponse {
    pub profile_visibility: bool,
    pub show_funding_history: bool,
    pub show_prediction_history: bool,
    pub two_factor_enabled: bool,
}

// Wallet Settings
#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserWalletSettingsRequest {
    pub wallet_address: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserWalletSettingsResponse {
    pub wallet_address: Option<String>,
}

// Preferences Settings
#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferencesSettingsRequest {
    pub dark_mode: bool,
    pub language: String,
    pub timezone: String,
    pub display_currency: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferencesSettingsResponse {
    pub dark_mode: bool,
    pub language: String,
    pub timezone: String,
    pub display_currency: String,
}

// Complete Settings Response (all tabs)
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAllSettingsResponse {
    pub profile: UserProfileSettingsResponse,
    pub notifications: UserNotificationSettingsResponse,
    pub privacy: UserPrivacySettingsResponse,
    pub wallet: UserWalletSettingsResponse,
    pub preferences: UserPreferencesSettingsResponse,
}
