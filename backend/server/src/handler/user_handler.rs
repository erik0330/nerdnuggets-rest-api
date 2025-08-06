use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use types::dto::{
    ChangeRoleRequest, GetEditorsOption, LoginAndRegisterResponse, OffsetAndLimitOption,
    UserAllSettingsResponse, UserCheckResponse, UserCheckUsernameOption,
    UserNotificationSettingsRequest, UserNotificationSettingsResponse, UserOnboardingRequest,
    UserPreferencesSettingsRequest, UserPreferencesSettingsResponse, UserPrivacySettingsRequest,
    UserPrivacySettingsResponse, UserProfileResponse, UserProfileSettingsRequest,
    UserProfileSettingsResponse, UserWalletSettingsRequest, UserWalletSettingsResponse,
};
use types::error::UserError;
use types::models::{ActivityHistory, User, UserInfo};
use types::{
    dto::UserReadDto,
    error::{ApiError, ValidatedRequest},
};

pub async fn get_user(Extension(user): Extension<User>) -> Result<Json<UserReadDto>, ApiError> {
    Ok(Json(UserReadDto::from(user)))
}

pub async fn get_editors(
    Extension(user): Extension<User>,
    Query(opts): Query<GetEditorsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserInfo>>, ApiError> {
    let mut users = state
        .service
        .user
        .get_editors(opts.offset, opts.limit, opts.name)
        .await?;
    users.retain(|u| u.id != user.id);
    Ok(Json(users))
}

pub async fn update_user_onboarding(
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserOnboardingRequest>,
) -> Result<Json<UserReadDto>, ApiError> {
    let user = state
        .service
        .user
        .update_user_onboarding(&id, payload)
        .await?;
    return Ok(Json(UserReadDto::from(user)));
}

pub async fn change_role(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ChangeRoleRequest>,
) -> Result<Json<LoginAndRegisterResponse>, ApiError> {
    if !user.roles.contains(&payload.role) {
        return Err(ApiError::UserError(UserError::RoleNotAllowed))?;
    }
    return Ok(Json(LoginAndRegisterResponse {
        user: UserReadDto::from(user.to_owned()),
        token: state.service.token.generate_token(user, payload.role)?,
    }));
}

pub async fn get_my_activities(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<ActivityHistory>>, ApiError> {
    let res = state
        .service
        .user
        .get_activities(Some(user.id), opts.offset, opts.limit)
        .await?;
    Ok(Json(res))
}

pub async fn check_username(
    Query(opts): Query<UserCheckUsernameOption>,
    State(state): State<AppState>,
) -> Result<Json<UserCheckResponse>, ApiError> {
    let username = opts.username.unwrap_or_default();
    if username.is_empty() {
        return Ok(Json(UserCheckResponse {
            is_available: false,
        }));
    }

    // Validate username format: only alphanumeric (letters and numbers)
    if !username.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Ok(Json(UserCheckResponse {
            is_available: false,
        }));
    }

    let res = state.service.user.check_username(&username).await?;
    Ok(Json(res))
}

// pub async fn update_username(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateUsernameRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let onboarding_step = match user.onboarding_step {
//         Some(2) => 3,
//         Some(step) => step,
//         None => 3,
//     };
//     // if let Some(user_name_updated_at) = user.user_name_updated_at {
//     //     let duration = Utc::now()
//     //         .naive_utc()
//     //         .date()
//     //         .signed_duration_since(user_name_updated_at.naive_utc().date());
//     //     if duration < Duration::days(30) {
//     //         return Err(ApiError::UserError(UserError::SomethingWentWrong(format!(
//     //             "It's been {} days since you changed your username",
//     //             duration.num_days()
//     //         ))))?;
//     //     }
//     // }
//     let result = state
//         .service
//         .user
//         .update_username(user.id, payload, onboarding_step)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn update_profile(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateProfileRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state.service.user.update_profile(user.id, payload).await?;
//     Ok(Json(result))
// }

// pub async fn upload_avatar(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     mut multipart: Multipart,
// ) -> Result<Json<UserUploadAvatarResponse>, ApiError> {
//     let bucket_name = state.env.aws_bucket_name;

//     let field = multipart
//         .next_field()
//         .await
//         .map_err(|_| ApiError::UploadError(UploadError::FailedProcessData))?
//         .ok_or(ApiError::UploadError(UploadError::NoFileProvided))?;

//     let content_type = field.content_type().unwrap_or_default().to_owned();
//     let key = format!("avatar/{}", Uuid::new_v4().to_string());
//     let url = format!("https://{bucket_name}/{key}");

//     let bytes = field.bytes().await.map_err(|_| {
//         ApiError::UploadError(UploadError::SomethingWentWrong(
//             "Failed to read file data".to_string(),
//         ))
//     })?;

//     let body = ByteStream::from(bytes.to_vec());
//     state
//         .s3_client
//         .put_object()
//         .bucket(&bucket_name)
//         .content_type(&content_type)
//         .content_length(bytes.len() as i64)
//         .key(&key)
//         .body(body)
//         .send()
//         .await
//         .map_err(|_| {
//             ApiError::UploadError(UploadError::SomethingWentWrong(
//                 "Failed to upload file to S3".to_string(),
//             ))
//         })?;

//     let result = state
//         .service
//         .user
//         .update_avatar_url(user.id, Some(url))
//         .await?;

//     if let Some(url) = user.avatar_url {
//         if let Some(url) = Url::parse(&url).ok() {
//             let key = url.path().trim_start_matches('/').to_string();
//             state
//                 .s3_client
//                 .delete_object()
//                 .bucket(&bucket_name)
//                 .key(key)
//                 .send()
//                 .await
//                 .ok();
//         }
//     }
//     Ok(Json(result))
// }

// pub async fn remove_avatar(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if let Some(avatar) = user.avatar_url {
//         let bucket_name = state.env.aws_bucket_name;
//         let _result = state.service.user.update_avatar_url(user.id, None).await?;
//         if let Some(url) = Url::parse(&avatar).ok() {
//             let key = url.path().trim_start_matches('/').to_string();
//             state
//                 .s3_client
//                 .delete_object()
//                 .bucket(&bucket_name)
//                 .key(key)
//                 .send()
//                 .await
//                 .ok();
//         }
//     }
//     Ok(Json(UserUpdateResponse { state: true }))
// }

// pub async fn get_profile(
//     Extension(me): Extension<Option<User>>,
//     opts: Option<Query<GetProfileOption>>,
//     State(state): State<AppState>,
// ) -> Result<Json<UserGetProfileResponse>, ApiError> {
//     let Query(opts) = opts.unwrap_or_default();
//     let my_name = me.clone().map(|u| u.user_name.unwrap_or_default());
//     let user_name = opts
//         .user_id
//         .filter(|s| !s.is_empty())
//         .or_else(|| my_name.clone())
//         .ok_or(UserError::UserNotFound)?;
//     let user = state
//         .service
//         .user
//         .find_by_user_name(
//             &user_name,
//             !opts.session.unwrap_or_default().is_empty()
//                 && user_name != my_name.clone().unwrap_or_default(),
//         )
//         .await?;
//     if user.is_suspended.unwrap_or_default() {
//         return Err(UserError::SomethingWentWrong(
//             "This user account has been suspended".to_string(),
//         ))?;
//     }
//     let user_id = user.id;
//     let affiliations = state.service.user.get_affiliations(user_id).await?;
//     let is_following = state
//         .service
//         .user
//         .check_user_related(me.as_ref().map(|u| u.id), user_id, 0)
//         .await?;
//     let is_follower = state
//         .service
//         .user
//         .check_user_related(me.as_ref().map(|u| u.id), user_id, 1)
//         .await?;
//     let is_blocked = state
//         .service
//         .user
//         .check_user_related(me.as_ref().map(|u| u.id), user_id, 2)
//         .await?;
//     let is_reported = state
//         .service
//         .user
//         .check_user_related(me.as_ref().map(|u| u.id), user_id, 3)
//         .await?;
//     let followers = state
//         .service
//         .user
//         .get_members(Some(user_id), me.as_ref().map(|u| u.id), "", 1, 0, 3)
//         .await?;
//     let degrees = state
//         .service
//         .util
//         .get_degrees_by_ids(&user.degree.unwrap_or_default())
//         .await
//         .unwrap_or_default();
//     return Ok(Json(UserGetProfileResponse {
//         id: user.id,
//         noble_id: user.noble_id,
//         first_name: user.first_name,
//         middle_name: user.middle_name,
//         second_name: user.second_name,
//         user_name: user.user_name,
//         avatar: user.avatar_url,
//         gender: user.gender,
//         country: user.country,
//         city: user.city,
//         degree: degrees,
//         about_me: user.about_me,
//         expand_about_me: user.expand_about_me,
//         affiliations,
//         wallpaper: user.wallpaper_url,
//         count_followers: state.service.user.count_followers(user_id).await?,
//         count_following: state.service.user.count_following(user_id).await?,
//         count_view: user.count_view.unwrap_or_default() as u64,
//         is_following,
//         is_follower,
//         is_blocked,
//         is_reported,
//         followers,
//         joined_at: datetime_to_string(user.created_at),
//         gmail: user.gmail,
//         principal: user.principal,
//         twitter_username: user.twitter_username,
//         telegram_username: user.telegram_username,
//         web_site: user.web_site,
//         linkedin: user.linkedin,
//     }));
// }

// pub async fn get_settings(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
// ) -> Result<Json<UserGetSettingsResponse>, ApiError> {
//     let affiliations = state.service.user.get_affiliations(user.id).await?;
//     let degree = state
//         .service
//         .util
//         .get_degrees_by_ids(&user.degree.unwrap_or_default())
//         .await?;
//     return Ok(Json(UserGetSettingsResponse {
//         id: user.id,
//         noble_id: user.noble_id,
//         first_name: user.first_name,
//         middle_name: user.middle_name,
//         second_name: user.second_name,
//         user_name: user.user_name,
//         avatar: user.avatar_url,
//         gender: user.gender,
//         country: user.country,
//         city: user.city,
//         degree_list: degree,
//         about_me: user.about_me,
//         expand_about_me: user.expand_about_me,
//         affiliations,
//         wallpaper: user.wallpaper_url,
//         domain_expertise: UserUpdateDomainExpertiseRequest {
//             expertise_domains: user.expertise_domains,
//             years_of_experience: user.years_of_experience,
//             years_of_experiences: user.years_of_experiences,
//         },
//         nobleblocks_role: UserUpdateNobleblocksRoleRequest {
//             r_is_like: user.r_is_like,
//             r_expertise_domains: user.r_expertise_domains,
//             r_number_review: user.r_number_review,
//             r_is_before_journals: user.r_is_before_journals,
//             r_journals: user.r_journals,
//             r_is_open: user.r_is_open,
//             r_number: user.r_number,
//             r_review_style: user.r_review_style,
//             e_is_like: user.e_is_like,
//             e_years: user.e_years,
//             e_is_before_journals: user.e_is_before_journals,
//             e_journals: user.e_journals,
//             e_is_open: user.e_is_open,
//             e_number: user.e_number,
//             e_decision_making: user.e_decision_making,
//             c_is_like: user.c_is_like,
//             c_years: user.c_years,
//             c_article_types: user.c_article_types,
//             c_formatting_styles: user.c_formatting_styles,
//             c_number: user.c_number,
//         },
//     }));
// }

// pub async fn update_setting_profile(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateSettingProfileRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state
//         .service
//         .user
//         .update_setting_profile(user.id, payload)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn get_affiliations(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<Affiliation>>, ApiError> {
//     let result = state.service.user.get_affiliations(user.id).await?;
//     Ok(Json(result))
// }

// pub async fn add_affiliation(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserAddAffiliationRequest>,
// ) -> Result<Json<UserAddAffiliationResponse>, ApiError> {
//     let result = state.service.user.add_affiliation(user, payload).await?;
//     Ok(Json(result))
// }

// pub async fn edit_affiliation(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserEditAffiliationRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state.service.user.edit_affiliation(payload).await?;
//     Ok(Json(result))
// }

// pub async fn delete_affiliation(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserDeleteRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state.service.user.delete_affiliation(payload).await?;
//     Ok(Json(result))
// }

// pub async fn update_domain_expertise(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateDomainExpertiseRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state
//         .service
//         .user
//         .edit_domain_expertise(user.id, payload)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn update_nobleblocks_role(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateNobleblocksRoleRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state
//         .service
//         .user
//         .update_nobleblocks_role(user, payload)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn get_roles(
//     Extension(user): Extension<User>,
// ) -> Result<Json<UserGetRolesResponse>, ApiError> {
//     Ok(Json(UserGetRolesResponse {
//         role_editor: user.role_editor.unwrap_or_default(),
//         role_reviewer: user.role_reviewer.unwrap_or_default(),
//         role_copy_editor: user.role_copy_editor.unwrap_or_default(),
//         role_bounty_hunter: user.role_bounty_hunter.unwrap_or_default(),
//     }))
// }

// pub async fn update_roles(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateRolesRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state.service.user.update_roles(user.id, payload).await?;
//     Ok(Json(result))
// }

// pub async fn get_notification(
//     Extension(user): Extension<User>,
// ) -> Result<Json<UserGetNotificationResponse>, ApiError> {
//     Ok(Json(UserGetNotificationResponse {
//         on_notification: user.on_notification.unwrap_or_default(),
//     }))
// }

// pub async fn update_notification(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateNotificationRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     let result = state
//         .service
//         .user
//         .update_notification(user.id, payload)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn connect_google(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserConnectGoogleRequest>,
// ) -> Result<Json<UserConnectGoogleResponse>, ApiError> {
//     let google_user = get_google_user(&payload.access_token).await;

//     if let Err(_) = google_user {
//         return Err(DbError::SomethingWentWrong(
//             "An error occurred while trying to retrieve user information.".to_string(),
//         ))?;
//     }

//     let google_user = google_user.unwrap();
//     let email = google_user.email.to_lowercase();
//     if let Ok(u) = state.service.user.find_by_gmail(&email).await {
//         if u.id != user.id {
//             return Err(UserError::SomethingWentWrong(
//                 "This gmail is already in use.".to_string(),
//             ))?;
//         } else {
//             return Ok(Json(UserConnectGoogleResponse { google: email }));
//         }
//     }

//     if let Ok(u) = state.service.user.find_by_email(&email).await {
//         if u.id != user.id {
//             return Err(UserError::SomethingWentWrong(
//                 "This gmail is already in use.".to_string(),
//             ))?;
//         }
//     }
//     if !state
//         .service
//         .user
//         .update_gmail(user.id, Some(email.clone()))
//         .await?
//     {
//         return Err(UserError::SomethingWentWrong(
//             "Connect google failed.".to_string(),
//         ))?;
//     }
//     state
//         .service
//         .user
//         .update_rating(user.id, RATING_CONNECT_SOCIAL, true)
//         .await?;

//     Ok(Json(UserConnectGoogleResponse { google: email }))
// }

// pub async fn update_email(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateEmailRequest>,
// ) -> Result<Json<UserSendPasskeyResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::SomethingWentWrong(
//             "The email is invalid".to_string(),
//         )));
//     }
//     if state
//         .service
//         .user
//         .find_by_email(&payload.email)
//         .await
//         .is_ok()
//     {
//         return Err(UserError::EmailAlreadyUsed)?;
//     }
//     if payload.is_new.unwrap_or_default() {
//         let len = payload.password.clone().unwrap_or_default().len();
//         if len < 8 || len > 20 {
//             return Err(UserError::InvalidPassword)?;
//         }
//     }
//     let now = state.env.now();
//     let iat = now.timestamp();
//     let exp = now
//         .checked_add_signed(Duration::seconds(state.env.email_verify_exp_second))
//         .unwrap()
//         .timestamp();
//     let passkey = state.env.generate_passkey().to_string();
//     let (verify_type, password) = if payload.is_new.unwrap_or_default() {
//         (
//             EmailVerifyType::AddEmail,
//             payload.password.unwrap_or_default(),
//         )
//     } else {
//         (
//             EmailVerifyType::VerifyEmail,
//             user.password.unwrap_or_default(),
//         )
//     };
//     let try_limit = state.env.email_verify_limit;

//     let is_sent_passkey =
//         if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
//             if iat < temp_user.iat.unwrap_or_default() + EMAIL_SEND_AGAIN_IN_SECONDS {
//                 return Err(UserError::CantSendEmail)?;
//             }
//             state
//                 .service
//                 .user
//                 .update_tempuser_with_email(
//                     &payload.email,
//                     &password,
//                     &verify_type.to_string(),
//                     &passkey,
//                     try_limit,
//                     iat,
//                     exp,
//                     now,
//                 )
//                 .await
//                 .map_err(|_| {
//                     ApiError::DbError(DbError::SomethingWentWrong(
//                         "TempUser can't be updated".to_string(),
//                     ))
//                 })?
//         } else {
//             state
//                 .service
//                 .user
//                 .create_tempuser_with_email(
//                     &payload.email,
//                     &password,
//                     &verify_type.to_string(),
//                     &passkey,
//                     try_limit,
//                     iat,
//                     exp,
//                     now,
//                 )
//                 .await
//                 .map_err(|_| {
//                     ApiError::DbError(DbError::SomethingWentWrong(
//                         "TempUser can't be created".to_string(),
//                     ))
//                 })?
//         };

//     if !send_auth_email(
//         payload.email.to_owned(),
//         passkey.to_owned(),
//         verify_type,
//         &state.ses_client,
//     )
//     .await
//     {
//         return Err(UserError::CantSendEmail)?;
//     }

//     Ok(Json(UserSendPasskeyResponse {
//         is_sent_passkey,
//         iat,
//         exp,
//     }))
// }

// pub async fn connect_website(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserConnectWebSiteRequest>,
// ) -> Result<Json<UserConnectWebSiteResponse>, ApiError> {
//     if !payload.web_site.is_empty() {
//         if let Ok(u) = state.service.user.find_by_website(&payload.web_site).await {
//             if u.id != user.id {
//                 return Err(UserError::SomethingWentWrong(
//                     "This website is already in use.".to_string(),
//                 ))?;
//             } else {
//                 return Ok(Json(UserConnectWebSiteResponse {
//                     web_site: payload.web_site,
//                 }));
//             }
//         }
//     }
//     if !state
//         .service
//         .user
//         .update_website(user.id, Some(payload.web_site.clone()))
//         .await?
//     {
//         return Err(UserError::SomethingWentWrong(
//             "Connect website failed.".to_string(),
//         ))?;
//     }
//     state
//         .service
//         .user
//         .update_rating(user.id, RATING_CONNECT_SOCIAL, true)
//         .await?;
//     Ok(Json(UserConnectWebSiteResponse {
//         web_site: payload.web_site,
//     }))
// }

// pub async fn resend_email_verify_code(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdateEmailRequest>,
// ) -> Result<Json<UserSendPasskeyResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::SomethingWentWrong(
//             "The email is invalid".to_string(),
//         )));
//     }
//     if state
//         .service
//         .user
//         .find_by_email(&payload.email)
//         .await
//         .is_ok()
//     {
//         return Err(UserError::EmailAlreadyUsed)?;
//     }
//     if payload.is_new.unwrap_or_default() {
//         let len = payload.password.clone().unwrap_or_default().len();
//         if len < 8 || len > 20 {
//             return Err(UserError::InvalidPassword)?;
//         }
//     }
//     if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
//         let now = state.env.now();
//         let iat = now.timestamp();
//         let exp = now
//             .checked_add_signed(Duration::seconds(state.env.email_verify_exp_second))
//             .unwrap()
//             .timestamp();
//         let passkey = state.env.generate_passkey().to_string();
//         let (verify_type, password) = if payload.is_new.unwrap_or_default() {
//             (
//                 EmailVerifyType::AddEmail,
//                 payload.password.unwrap_or_default(),
//             )
//         } else {
//             (
//                 EmailVerifyType::VerifyEmail,
//                 user.password.unwrap_or_default(),
//             )
//         };
//         let try_limit = temp_user.try_limit.unwrap_or_default();
//         if iat < temp_user.iat.unwrap_or_default() + EMAIL_SEND_AGAIN_IN_SECONDS {
//             return Err(UserError::CantSendEmail)?;
//         }
//         if !send_auth_email(
//             payload.email.to_owned(),
//             passkey.to_owned(),
//             verify_type.clone(),
//             &state.ses_client,
//         )
//         .await
//         {
//             return Err(UserError::CantSendEmail)?;
//         }
//         let is_sent_passkey = state
//             .service
//             .user
//             .update_tempuser_with_email(
//                 &temp_user.email.unwrap_or_default(),
//                 &password,
//                 &verify_type.to_string(),
//                 &passkey,
//                 try_limit,
//                 iat,
//                 exp,
//                 now,
//             )
//             .await
//             .map_err(|_| {
//                 ApiError::DbError(DbError::SomethingWentWrong(
//                     "TempUser can't be updated".to_string(),
//                 ))
//             })?;
//         return Ok(Json(UserSendPasskeyResponse {
//             is_sent_passkey,
//             iat,
//             exp,
//         }));
//     }
//     Err(UserError::TempUserNotFound)?
// }

// pub async fn verify_passkey_change_email(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserVerifyPasskeyRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::SomethingWentWrong(
//             "The email is invalid".to_string(),
//         )));
//     }
//     if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
//         if temp_user.exp.unwrap_or(0) < state.env.now().timestamp() {
//             return Err(UserError::ExpiredPasskey)?;
//         }
//         let passkey = payload.passkey.unwrap_or_default();
//         if passkey.is_empty() || !temp_user.passkey.unwrap_or_default().eq(&passkey) {
//             return Err(UserError::InvalidPasskey)?;
//         }
//         let result = match temp_user.verify_type.unwrap_or_default().as_str() {
//             "AddEmail" => {
//                 state
//                     .service
//                     .user
//                     .update_email(
//                         user.id,
//                         payload.email,
//                         bcrypt::hash(temp_user.password.unwrap_or_default(), 12).unwrap(),
//                     )
//                     .await?
//             }
//             "VerifyEmail" => {
//                 state
//                     .service
//                     .user
//                     .update_email(user.id, payload.email, user.password.unwrap_or_default())
//                     .await?
//             }
//             _ => UserUpdateResponse { state: false },
//         };
//         return Ok(Json(result));
//     }
//     Err(UserError::TempUserNotFound)?
// }

// pub async fn send_email_verify_code_to_reset_password(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserSendVerifyCodeResetPwdRequest>,
// ) -> Result<Json<UserSendPasskeyResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::SomethingWentWrong(
//             "The email is invalid".to_string(),
//         )));
//     }
//     if user.email.clone().unwrap() != payload.email {
//         return Err(UserError::EmailNotMatch)?;
//     }
//     if user.email.is_none() || !user.verified_email.unwrap_or_default() {
//         return Err(UserError::CantSendEmail)?;
//     }
//     let now = state.env.now();
//     let iat = now.timestamp();
//     let exp = now
//         .checked_add_signed(Duration::seconds(state.env.email_verify_exp_second))
//         .unwrap()
//         .timestamp();
//     let passkey = state.env.generate_passkey().to_string();
//     let verify_type = EmailVerifyType::ResetPassword.to_string();
//     let try_limit = state.env.email_verify_limit;

//     let is_sent_passkey = if let Ok(temp_user) = state
//         .service
//         .user
//         .tempuser_by_email(&user.email.clone().unwrap_or_default())
//         .await
//     {
//         if iat < temp_user.iat.unwrap_or_default() + EMAIL_SEND_AGAIN_IN_SECONDS {
//             return Err(UserError::CantSendEmail)?;
//         }
//         state
//             .service
//             .user
//             .update_tempuser_with_email(
//                 &user.email.clone().unwrap_or_default(),
//                 &user.password.unwrap_or_default(),
//                 &verify_type,
//                 &passkey,
//                 try_limit,
//                 iat,
//                 exp,
//                 now,
//             )
//             .await
//             .map_err(|_| {
//                 ApiError::DbError(DbError::SomethingWentWrong(
//                     "TempUser can't be updated".to_string(),
//                 ))
//             })?
//     } else {
//         state
//             .service
//             .user
//             .create_tempuser_with_email(
//                 &user.email.clone().unwrap_or_default(),
//                 &user.password.unwrap_or_default(),
//                 &verify_type,
//                 &passkey,
//                 try_limit,
//                 iat,
//                 exp,
//                 now,
//             )
//             .await
//             .map_err(|_| {
//                 ApiError::DbError(DbError::SomethingWentWrong(
//                     "TempUser can't be created".to_string(),
//                 ))
//             })?
//     };
//     if !send_auth_email(
//         user.email.unwrap_or_default(),
//         passkey.to_owned(),
//         EmailVerifyType::ResetPassword,
//         &state.ses_client,
//     )
//     .await
//     {
//         return Err(UserError::CantSendEmail)?;
//     }

//     Ok(Json(UserSendPasskeyResponse {
//         is_sent_passkey,
//         iat,
//         exp,
//     }))
// }

// pub async fn resend_email_verify_code_to_reset_password(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserResendVerifyCodeResetPwdRequest>,
// ) -> Result<Json<UserSendPasskeyResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::SomethingWentWrong(
//             "The email is invalid".to_string(),
//         )));
//     }
//     if user.email.is_none() || !user.verified_email.unwrap_or_default() {
//         return Err(UserError::CantSendEmail)?;
//     }
//     if user.email.clone().unwrap() != payload.email {
//         return Err(UserError::EmailNotMatch)?;
//     }
//     if let Ok(temp_user) = state
//         .service
//         .user
//         .tempuser_by_email(&user.email.clone().unwrap_or_default())
//         .await
//     {
//         let now = state.env.now();
//         let iat = now.timestamp();
//         let exp = now
//             .checked_add_signed(Duration::seconds(state.env.email_verify_exp_second))
//             .unwrap()
//             .timestamp();
//         let passkey = state.env.generate_passkey().to_string();
//         let verify_type = temp_user.verify_type.unwrap_or_default();
//         let try_limit = temp_user.try_limit.unwrap_or_default();
//         if iat < temp_user.iat.unwrap_or_default() + EMAIL_SEND_AGAIN_IN_SECONDS {
//             return Err(UserError::CantSendEmail)?;
//         }
//         if !send_auth_email(
//             user.email.unwrap_or_default(),
//             passkey.to_owned(),
//             EmailVerifyType::ResetPassword,
//             &state.ses_client,
//         )
//         .await
//         {
//             return Err(UserError::CantSendEmail)?;
//         }
//         let is_sent_passkey = state
//             .service
//             .user
//             .update_tempuser_with_email(
//                 &temp_user.email.unwrap_or_default(),
//                 &temp_user.password.unwrap_or_default(),
//                 &verify_type,
//                 &passkey,
//                 try_limit,
//                 iat,
//                 exp,
//                 now,
//             )
//             .await
//             .map_err(|_| {
//                 ApiError::DbError(DbError::SomethingWentWrong(
//                     "TempUser can't be updated".to_string(),
//                 ))
//             })?;
//         return Ok(Json(UserSendPasskeyResponse {
//             is_sent_passkey,
//             iat,
//             exp,
//         }));
//     }
//     Err(UserError::TempUserNotFound)?
// }

// pub async fn verify_passkey_reset_password(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserVerifyPasskeyResetPwdRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if user.email.is_none() || !user.verified_email.unwrap_or_default() {
//         return Err(UserError::TempUserNotFound)?;
//     }
//     if let Ok(temp_user) = state
//         .service
//         .user
//         .tempuser_by_email(&user.email.clone().unwrap_or_default())
//         .await
//     {
//         if temp_user.exp.unwrap_or(0) < state.env.now().timestamp() {
//             return Err(UserError::ExpiredPasskey)?;
//         }
//         let passkey = payload.passkey.unwrap_or_default();
//         if passkey.is_empty()
//             || !temp_user
//                 .passkey
//                 .to_owned()
//                 .unwrap_or_default()
//                 .eq(&passkey)
//         {
//             return Err(UserError::InvalidPasskey)?;
//         }

//         let now = state.env.now();
//         let exp = now
//             .checked_add_signed(Duration::minutes(state.env.email_verify_exp_second))
//             .unwrap()
//             .timestamp();
//         state
//             .service
//             .user
//             .update_tempuser_with_email(
//                 &temp_user.email.unwrap_or_default(),
//                 &temp_user.password.unwrap_or_default(),
//                 &temp_user.verify_type.unwrap_or_default(),
//                 &temp_user.passkey.unwrap_or_default(),
//                 temp_user.try_limit.unwrap_or_default(),
//                 temp_user.iat.unwrap_or_default(),
//                 exp,
//                 now,
//             )
//             .await
//             .map_err(|_| {
//                 ApiError::DbError(DbError::SomethingWentWrong(
//                     "TempUser can't be updated".to_string(),
//                 ))
//             })?;
//         return Ok(Json(UserUpdateResponse { state: true }));
//     }
//     Err(UserError::TempUserNotFound)?
// }

// pub async fn update_password(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserUpdatePasswordRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if let Ok(temp_user) = state
//         .service
//         .user
//         .tempuser_by_email(&user.email.clone().unwrap_or_default())
//         .await
//     {
//         let passkey = payload.passkey;
//         if passkey.is_empty()
//             || !temp_user
//                 .passkey
//                 .to_owned()
//                 .unwrap_or_default()
//                 .eq(&passkey)
//         {
//             return Err(UserError::InvalidPasskey)?;
//         }
//         if temp_user.exp.unwrap_or(0) < state.env.now().timestamp() {
//             return Err(UserError::ExpiredPasskey)?;
//         }
//         let result = state
//             .service
//             .user
//             .update_password(user.id, payload.old_pwd, payload.new_pwd)
//             .await?;

//         let now = state.env.now();
//         let exp = now
//             .checked_add_signed(Duration::seconds(1))
//             .unwrap()
//             .timestamp();
//         state
//             .service
//             .user
//             .update_tempuser_with_email(
//                 &temp_user.email.unwrap_or_default(),
//                 &temp_user.password.unwrap_or_default(),
//                 &temp_user.verify_type.unwrap_or_default(),
//                 &temp_user.passkey.unwrap_or_default(),
//                 temp_user.try_limit.unwrap_or_default(),
//                 temp_user.iat.unwrap_or_default(),
//                 exp,
//                 now,
//             )
//             .await
//             .map_err(|_| {
//                 ApiError::DbError(DbError::SomethingWentWrong(
//                     "TempUser can't be updated".to_string(),
//                 ))
//             })?;
//         return Ok(Json(result));
//     }
//     Err(UserError::TempUserNotFound)?
// }

// pub async fn get_members(
//     Extension(me): Extension<Option<User>>,
//     opts: Option<Query<GetMembersOption>>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<UserInfo>>, ApiError> {
//     let Query(opts) = opts.unwrap_or_default();
//     let my_name = me.clone().map(|u| u.user_name.unwrap_or_default());
//     let user_name = opts
//         .user_name
//         .filter(|s| !s.is_empty())
//         .or_else(|| my_name.clone());

//     let user = if let Some(user_name) = user_name {
//         Some(
//             state
//                 .service
//                 .user
//                 .find_by_user_name(&user_name, false)
//                 .await?,
//         )
//     } else {
//         None
//     };

//     let result = state
//         .service
//         .user
//         .get_members(
//             user.map(|u| u.id),
//             me.map(|u| u.id),
//             &opts.name.unwrap_or_default(),
//             opts.user_type.unwrap_or(0),
//             opts.start.unwrap_or(0),
//             opts.limit.unwrap_or(10),
//         )
//         .await?;
//     Ok(Json(result))
// }

// pub async fn follow_user(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserFollowRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     let result = state
//         .service
//         .user
//         .follow_user(user.id, payload.followed_id)
//         .await?;
//     state
//         .service
//         .notification
//         .add_notification(
//             MessageType::One,
//             NotificationType::FollowUser,
//             &payload.followed_id,
//             &Some(user.id),
//             &None,
//         )
//         .await?;
//     state
//         .service
//         .user
//         .update_rating(user.id, RATING_LIKE, true)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn block_user(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserBlockRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     let result = if payload.status.unwrap_or_default() {
//         state
//             .service
//             .user
//             .block_user(user.id, payload.blocked_id, payload.reason)
//             .await?
//     } else {
//         state
//             .service
//             .user
//             .unblock_user(user.id, payload.blocked_id)
//             .await?
//     };
//     Ok(Json(result))
// }

// pub async fn get_suggested_user(
//     Extension(user): Extension<User>,
//     opts: Option<Query<GetSuggestedUserOption>>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<UserInfo>>, ApiError> {
//     let Query(opts) = opts.unwrap_or_default();
//     let result = state
//         .service
//         .user
//         .get_suggested_user(user.id, opts.count.unwrap_or(5))
//         .await?;
//     Ok(Json(result))
// }

// pub async fn get_users_by_ids(
//     opts: Option<Query<GetUsersByIdsOption>>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<UserInfo>>, ApiError> {
//     let Query(opts) = opts.unwrap_or_default();
//     let result = state.service.user.get_users_by_ids(opts.user_ids).await?;
//     Ok(Json(result))
// }

// pub async fn add_subscription(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<SubscriptionInfo>,
// ) -> Result<Json<bool>, ApiError> {
//     state
//         .service
//         .notification
//         .add_subscription(&user.id, &payload)
//         .await?;
//     Ok(Json(true))
// }

// pub async fn get_notifications(
//     Extension(user): Extension<User>,
//     opts: Option<Query<GetNotificationsOption>>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<NotificationInfo>>, ApiError> {
//     let Query(opts) = opts.unwrap_or_default();
//     let result = state
//         .service
//         .notification
//         .get_notifications_by_user_id(&user.id, opts.from_id, opts.limit, opts.read_status)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn mark_notification_as_read(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<MarkNotificationAsReadRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     let result = state
//         .service
//         .notification
//         .mark_notification_as_read(&user.id, payload.from_id, payload.to_id)
//         .await?;
//     Ok(Json(result))
// }

// pub async fn read_notification(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<ReadNotificationRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     let result = state
//         .service
//         .notification
//         .read_notification(payload.notification_id, &user.id)
//         .await?;
//     Ok(Json(result))
// }

// ========================= USER SETTINGS HANDLERS =========================

pub async fn get_user_settings(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<UserAllSettingsResponse>, ApiError> {
    let settings = state.service.user.get_all_user_settings(user.id).await?;
    Ok(Json(settings))
}

pub async fn update_profile_settings(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserProfileSettingsRequest>,
) -> Result<Json<UserProfileSettingsResponse>, ApiError> {
    let result = state
        .service
        .user
        .update_profile_settings(user.id, payload)
        .await?;
    Ok(Json(result))
}

pub async fn update_notification_settings(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserNotificationSettingsRequest>,
) -> Result<Json<UserNotificationSettingsResponse>, ApiError> {
    let result = state
        .service
        .user
        .update_notification_settings(user.id, payload)
        .await?;
    Ok(Json(result))
}

pub async fn update_privacy_settings(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserPrivacySettingsRequest>,
) -> Result<Json<UserPrivacySettingsResponse>, ApiError> {
    let result = state
        .service
        .user
        .update_privacy_settings(user.id, payload)
        .await?;
    Ok(Json(result))
}

pub async fn update_wallet_settings(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserWalletSettingsRequest>,
) -> Result<Json<UserWalletSettingsResponse>, ApiError> {
    let result = state
        .service
        .user
        .update_wallet_settings(user.id, payload)
        .await?;
    Ok(Json(result))
}

pub async fn update_preferences_settings(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserPreferencesSettingsRequest>,
) -> Result<Json<UserPreferencesSettingsResponse>, ApiError> {
    let result = state
        .service
        .user
        .update_preferences_settings(user.id, payload)
        .await?;
    Ok(Json(result))
}

pub async fn get_user_profile_by_username(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<UserProfileResponse>, ApiError> {
    let profile = state
        .service
        .user
        .get_user_profile_by_username(&username)
        .await?;
    Ok(Json(profile))
}
