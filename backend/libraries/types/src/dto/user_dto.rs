use super::token_dto::TokenReadDto;
use crate::{
    models::{Affiliation, Degree, MessageType, NotificationType, SpeechInfo, User, UserInfo},
    NerdNuggetsOAuth2AppName,
};
use chrono::{DateTime, Utc};
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
pub struct UserConnectGoogleRequest {
    pub access_token: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectGoogleResponse {
    pub google: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectWebSiteRequest {
    pub web_site: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectWebSiteResponse {
    pub web_site: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectLinkedinRequest {
    pub linkedin: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectLinkedinResponse {
    pub linkedin: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectOrcIdRequest {
    pub orc_id: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectOrcIdResponse {
    pub orc_id: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectGoogleScholarRequest {
    pub google_scholar: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectGoogleScholarResponse {
    pub google_scholar: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectTelegramRequest {
    pub auth_date: i64,
    pub first_name: String,
    pub hash: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct UserConnectTelegramResponse {
    pub telegram_username: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectInternetIdentityRequest {
    pub payload: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectInternetIdentityResponse {
    pub principal: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginWithInternetIdentityRequest {
    pub payload: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginWithTwitterRequest {
    pub redirect_url: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginWithTwitterResponse {
    pub url: String,
    pub state: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginWithTwitter2Request {
    pub state: String,
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
pub struct UserLoginAndRegisterResponse {
    pub user: UserReadDto,
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct UserCheckEmailOption {
    pub email: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct UserCheckUsernameOption {
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserCheckResponse {
    pub is_available: bool,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterWithEmailRequest {
    pub name: String,
    pub institution: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserOnboardingRequest {
    pub name: String,
    pub institution: String,
    pub bio: String,
    pub roles: Vec<String>,
    pub interests: Vec<String>,
    pub wallet_address: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserSendPasskeyAgainRequest {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSendPasskeyResponse {
    pub is_sent_passkey: bool,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserVerifyPasskeyRequest {
    pub email: String,
    pub passkey: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserSendEmailForgotPwdRequest {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserResetPwdPwdRequest {
    pub email: String,
    pub passkey: String,
    #[validate(length(
        min = 8,
        max = 20,
        message = "Password must be between 8 and 20 characters"
    ))]
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserChangeRoleRequest {
    pub role: Option<u16>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateUsernameRequest {
    #[validate(length(
        min = 5,
        max = 50,
        message = "Username must be between 5 and 50 characters"
    ))]
    pub user_name: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserSendVerifyCodeResetPwdRequest {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserResendVerifyCodeResetPwdRequest {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserVerifyPasskeyResetPwdRequest {
    pub passkey: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdatePasswordRequest {
    pub passkey: String,
    #[validate(length(
        min = 8,
        max = 20,
        message = "Old password must be between 8 and 20 characters"
    ))]
    pub old_pwd: String,
    #[validate(length(
        min = 8,
        max = 20,
        message = "New password must be between 8 and 20 characters"
    ))]
    pub new_pwd: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateIsActiveRequest {
    pub is_active: bool,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateRolesRequest {
    pub role_editor: Option<bool>,
    pub role_reviewer: Option<bool>,
    pub role_copy_editor: Option<bool>,
    pub role_bounty_hunter: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdatePublishingRequest {
    pub allowed_comments: bool,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateNotificationRequest {
    pub on_notification: bool,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateEmailRequest {
    pub email: String,
    pub password: Option<String>,
    pub is_new: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateProfileRequest {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub second_name: Option<String>,
    pub gender: Option<i16>,
    pub country: Option<String>,
    pub city: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateSettingProfileRequest {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub second_name: Option<String>,
    pub gender: Option<i16>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub degree: Option<Vec<Uuid>>,
    pub new_degree: Option<Vec<String>>,
    pub about_me: Option<String>,
    #[validate(length(max = 10000, message = "About me must be less than 10000 characters"))]
    pub expand_about_me: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserAddAffiliationRequest {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "Institution is required"))]
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
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserAddAffiliationResponse {
    pub state: bool,
    pub id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserEditAffiliationRequest {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "Institution is required"))]
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
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserDeleteRequest {
    pub id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserAddDomainExpertiseRequest {
    pub expertise_domain: String,
    pub years: i16,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserAddDomainExpertiseResponse {
    pub state: bool,
    pub id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateDomainExpertiseRequest {
    pub expertise_domains: Option<Vec<String>>,
    pub years_of_experience: Option<i16>,
    pub years_of_experiences: Option<Vec<i16>>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateNobleblocksRoleRequest {
    // reviewer role
    pub r_is_like: bool,
    pub r_expertise_domains: Option<Vec<String>>,
    pub r_number_review: Option<i16>,
    pub r_is_before_journals: Option<bool>,
    pub r_journals: Option<Vec<String>>,
    pub r_is_open: Option<bool>,
    pub r_number: Option<i16>,
    pub r_review_style: Option<Vec<i16>>,
    // editor role
    pub e_is_like: bool,
    pub e_years: Option<i16>,
    pub e_is_before_journals: Option<bool>,
    pub e_journals: Option<Vec<String>>,
    pub e_is_open: Option<bool>,
    pub e_number: Option<i16>,
    pub e_decision_making: Option<bool>,
    // copy-editor role
    pub c_is_like: bool,
    pub c_years: Option<i16>,
    pub c_article_types: Option<Vec<Uuid>>,
    pub c_formatting_styles: Option<Vec<String>>,
    pub c_number: Option<i16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdateResponse {
    pub state: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUploadAvatarResponse {
    pub state: bool,
    pub avatar_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserUpdateWallPaperRequest {
    pub wallpaper_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUploadWallPaperResponse {
    pub state: bool,
    pub wallpaper_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserGetProfileResponse {
    pub id: Uuid,
    pub noble_id: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub second_name: Option<String>,
    pub user_name: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<i16>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub degree: Vec<Degree>,
    pub about_me: Option<String>,
    pub expand_about_me: Option<String>,
    pub affiliations: Vec<Affiliation>,
    pub wallpaper: Option<String>,
    pub count_followers: u64,
    pub count_following: u64,
    pub count_view: u64,
    pub is_following: bool,
    pub is_follower: bool,
    pub is_blocked: bool,
    pub is_reported: bool,
    pub followers: Vec<UserInfo>,
    pub joined_at: String,
    pub gmail: Option<String>,
    pub principal: Option<String>,
    pub twitter_username: Option<String>,
    pub telegram_username: Option<String>,
    pub web_site: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserGetSettingsResponse {
    pub id: Uuid,
    pub noble_id: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub second_name: Option<String>,
    pub user_name: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<i16>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub degree_list: Vec<Degree>,
    pub about_me: Option<String>,
    pub expand_about_me: Option<String>,
    pub affiliations: Vec<Affiliation>,
    pub wallpaper: Option<String>,
    pub domain_expertise: UserUpdateDomainExpertiseRequest,
    pub nobleblocks_role: UserUpdateNobleblocksRoleRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserGetRolesResponse {
    pub role_editor: bool,
    pub role_reviewer: bool,
    pub role_copy_editor: bool,
    pub role_bounty_hunter: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserGetPublishingResponse {
    pub allowed_comments: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserGetNotificationResponse {
    pub on_notification: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserGetSocialResponse {
    pub noble_id: Option<String>,
    pub principal: Option<String>,
    pub google: Option<String>,
    pub twitter_username: Option<String>,
    pub telegram_username: Option<String>,
    pub web_site: Option<String>,
    pub linkedin: Option<String>,
    pub orc_id: Option<String>,
    pub google_scholar: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserGetPrivacyResponse {
    pub email: String,
    pub user_name: String,
    pub password_updated_at: String,
    pub is_active: bool,
    pub count_blocked_users: u64,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetMembersOption {
    pub name: Option<String>,
    pub user_type: Option<u16>, // 0: All, 1: Follower, 2: Following, 3: Blocklist
    pub start: Option<i32>,
    pub limit: Option<i32>,
    pub user_name: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetProfileOption {
    pub user_id: Option<String>,
    pub session: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserFollowRequest {
    pub followed_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserReportRequest {
    pub reported_id: Uuid,
    pub description: Option<String>,
    pub status: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserBlockRequest {
    pub blocked_id: Uuid,
    pub reason: Option<String>,
    pub status: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct MarkNotificationAsReadRequest {
    pub from_id: Option<i64>,
    pub to_id: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct ReadNotificationRequest {
    pub notification_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetSuggestedUserOption {
    pub count: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetUsersByIdsOption {
    pub user_ids: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetNotificationsOption {
    pub from_id: Option<i64>,
    pub limit: Option<i32>,
    pub read_status: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AddUsersRequest {
    pub api_key: Option<String>,
    pub noble_id: Option<String>,
    pub user_name: Option<String>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub gender: Option<i16>,
    pub degree: Option<Vec<String>>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub avatar_url: Option<String>,
    pub about_me: Option<String>,
    pub principal: Option<String>,
    pub gmail: Option<String>,
    pub date_created: Option<DateTime<Utc>>,
    pub date_updated: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AddUsersResponse {
    pub id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AddUserFollowerRequest {
    pub api_key: Option<String>,
    pub follower: Uuid,
    pub following: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UpdatePaperTableRequest {
    pub api_key: Option<String>,
    pub post_id: Option<Uuid>,
    pub ai_error_json: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AddNotification {
    pub api_key: Option<String>,
    pub user_id: Uuid,
    pub message_type: MessageType,
    pub notification_type: NotificationType,
    pub referrer_id: Option<Uuid>,
    pub payload: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginWithNobleblocksParams {
    pub app_name: NerdNuggetsOAuth2AppName,
    pub redirect_url: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct NobleblocksOAuth2Params {
    pub app_name: NerdNuggetsOAuth2AppName,
    pub state: Uuid,
    pub redirect_url: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct NobleblocksOAuth2RedirectParams {
    pub state: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct GetSpeechesParams {
    pub start: Option<i64>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct GetSpeechesResponse {
    pub data: Vec<SpeechInfo>,
    pub total_count: i64,
    pub start: i64,
    pub length: i32,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct GetSpeechParams {
    pub speech_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserConnectStripePaymentRequest {
    pub payment_method_id: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserPayByStripe {
    pub payment_method_id: String,
    pub pay_type: i16, // StripePayType: 0: ArticleAIErrorCheck, 1: ArticleSubmit
    pub article_id: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct GetCardInfo {
    pub payment_method_id: String,
    pub brand: String,
    pub exp_month: i64,
    pub exp_year: i64,
    pub last4: String,
}
