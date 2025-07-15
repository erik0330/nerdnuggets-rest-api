use crate::state::AppState;
use axum::{
    extract::{Query, State},
    Json,
};
use chrono::Duration;
use third_party_api::google_oauth::get_google_user;
use types::{
    dto::{
        EmailVerificationResponse, LoginAndRegisterResponse, ResendVerificationEmailRequest,
        UserCheckEmailOption, UserCheckResponse, UserLoginWithEmailRequest,
        UserLoginWithGoogleRequest, UserReadDto, UserRegisterWithEmailRequest, VerifyEmailRequest,
    },
    error::{ApiError, DbError, UserError, ValidatedRequest},
    EmailVerifyType, UserRoleType,
};
use utils::{
    commons::{is_valid_email, send_auth_email},
    constants::EMAIL_SEND_AGAIN_IN_SECONDS,
};

pub async fn login_with_email(
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginWithEmailRequest>,
) -> Result<Json<LoginAndRegisterResponse>, ApiError> {
    if !is_valid_email(&payload.email) {
        return Err(ApiError::UserError(UserError::Str(
            "The email is invalid".to_string(),
        )));
    }
    let user = state.service.user.get_user_by_email(&payload.email).await?;

    if !user.verified_email {
        return Err(UserError::EmailNotVerified)?;
    }

    if state.service.user.verify_password(&user, &payload.password) {
        Ok(Json(LoginAndRegisterResponse {
            user: UserReadDto::from(user.to_owned()),
            token: state
                .service
                .token
                .generate_token(user, UserRoleType::Member.to_string())?,
        }))
    } else {
        Err(UserError::InvalidPassword)?
    }
}

pub async fn login_or_register_with_google(
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginWithGoogleRequest>,
) -> Result<Json<LoginAndRegisterResponse>, ApiError> {
    let google_user = get_google_user(&payload.access_token).await;

    if let Err(_) = google_user {
        return Err(DbError::Str(
            "An error occurred while trying to retrieve user information.".to_string(),
        ))?;
    }

    let google_user = google_user.unwrap();
    let email = google_user.email.to_lowercase();
    if let Ok(user) = state.service.user.get_user_by_gmail(&email).await {
        return Ok(Json(LoginAndRegisterResponse {
            user: UserReadDto::from(user.to_owned()),
            token: state
                .service
                .token
                .generate_token(user, UserRoleType::Member.to_string())?,
        }));
    }
    if let Ok(user) = state.service.user.get_user_by_email(&email).await {
        state
            .service
            .user
            .update_gmail(user.id, Some(email))
            .await?;
        return Ok(Json(LoginAndRegisterResponse {
            user: UserReadDto::from(user.to_owned()),
            token: state
                .service
                .token
                .generate_token(user, UserRoleType::Member.to_string())?,
        }));
    }
    match state.service.user.create_user_with_google(&email).await {
        Ok(user) => Ok(Json(LoginAndRegisterResponse {
            user: UserReadDto::from(user.to_owned()),
            token: state
                .service
                .token
                .generate_token(user, UserRoleType::Member.to_string())?,
        })),
        Err(_) => Err(ApiError::UserError(UserError::CantCreateUser)),
    }
}

// pub async fn login_or_register_with_twitter(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserLoginWithTwitterRequest>,
// ) -> Result<Json<UserLoginWithTwitterResponse>, ApiError> {
//     let mut ctx = state.ctx.lock().unwrap();
//     ctx.remove_expired_challenges();
//     let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
//     let (url, state) = ctx.client.auth_url(
//         challenge,
//         [
//             Scope::TweetRead,
//             Scope::TweetWrite,
//             Scope::UsersRead,
//             Scope::OfflineAccess,
//         ],
//     );
//     ctx.challenges.insert(
//         state.secret().clone(),
//         TwitterChallenge::new(verifier, payload.redirect_url, None),
//     );
//     Ok(Json(UserLoginWithTwitterResponse {
//         url: url.to_string(),
//         state: state.secret().clone(),
//     }))
// }

// pub async fn login_or_register_with_twitter2(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserLoginWithTwitter2Request>,
// ) -> Result<Json<UserLoginAndRegisterResponse>, ApiError> {
//     let twitter_id = {
//         let mut ctx = state.ctx.lock().unwrap();

//         if let Some(challenge) = ctx.challenges.remove(&payload.state) {
//             challenge.twitter_id
//         } else {
//             return Err(ApiError::UserError(UserError::UserNotFound));
//         }
//     };

//     let user = state.service.user.find_by_twitter_id(&twitter_id).await?;
//     return Ok(Json(UserLoginAndRegisterResponse {
//         user: UserReadDto::from(user.to_owned()),
//         token: state
//             .service
//             .token
//             .generate_token(user, UserRoleType::Guest)
//             .unwrap_or_default(),
//     }));
// }

// #[derive(Deserialize)]
// pub struct CallbackParams {
//     code: AuthorizationCode,
//     state: CsrfToken,
// }

// pub async fn twitter_oauth_callback(
//     State(s): State<AppState>,
//     Query(CallbackParams { code, state }): Query<CallbackParams>,
// ) -> impl IntoResponse {
//     let (client, verifier, redirect_url, user_id) = {
//         let ctx = s.ctx.lock().unwrap();

//         if let Some(challenge) = ctx.challenges.get(state.secret()) {
//             let client = ctx.client.clone();
//             (
//                 client,
//                 PkceCodeVerifier::new(challenge.verifier.secret().clone()),
//                 challenge.redirect_url.clone(),
//                 challenge.user_id.clone(),
//             )
//         } else {
//             return Err((
//                 StatusCode::BAD_REQUEST,
//                 "Invalid state returned".to_string(),
//             ));
//         }
//     };

//     let token = client
//         .request_token(code, PkceCodeVerifier::from(verifier))
//         .await
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

//     let api = TwitterApi::new(token);
//     let user = api
//         .get_users_me()
//         .send()
//         .await
//         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

//     match user.data.clone() {
//         Some(me) => {
//             if s.service
//                 .user
//                 .find_by_twitter_id(&me.id.to_string())
//                 .await
//                 .is_ok()
//             {
//                 if user_id.is_some() {
//                     return Err((
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         "Twitter id is already in use".to_string(),
//                     ));
//                 }
//             } else {
//                 if let Some(user_id) = user_id {
//                     s.service
//                         .user
//                         .update_twitter(user_id, Some(me.id.to_string()), Some(me.username))
//                         .await
//                         .unwrap_or_default();
//                 } else {
//                     s.service
//                         .user
//                         .create_user_with_twitter(&me.id.to_string(), &me.username)
//                         .await
//                         .unwrap_or_default();
//                 }
//             }
//             let mut ctx = s.ctx.lock().unwrap();
//             if let Some(challenge) = ctx.challenges.get_mut(state.secret()) {
//                 challenge.twitter_id = me.id.clone().to_string();
//             }
//         }
//         None => {
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "An error occurred while trying to retrieve user information.".to_string(),
//             ))
//         }
//     }

//     let redirect_url = if redirect_url.is_empty() {
//         String::new()
//     } else {
//         format!("?redirect_url={}", redirect_url)
//     };
//     let prefix_url = if user_id.is_some() {
//         "settings?tab=Social"
//     } else {
//         "login"
//     };
//     Ok(Redirect::to(&format!(
//         "{}/{}{}",
//         s.env.frontend_url, prefix_url, redirect_url
//     )))
// }

pub async fn check_email(
    opts: Option<Query<UserCheckEmailOption>>,
    State(state): State<AppState>,
) -> Result<Json<UserCheckResponse>, ApiError> {
    let Query(opts) = opts.unwrap_or_default();
    let res = state
        .service
        .user
        .check_email(&opts.email.unwrap_or_default())
        .await?;
    Ok(Json(res))
}

pub async fn register_with_email(
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UserRegisterWithEmailRequest>,
) -> Result<Json<EmailVerificationResponse>, ApiError> {
    if !is_valid_email(&payload.email) {
        return Err(ApiError::UserError(UserError::Str(
            "The email is invalid".to_string(),
        )));
    }
    if state
        .service
        .user
        .get_user_by_email(&payload.email)
        .await
        .is_ok()
    {
        return Err(UserError::UserAlreadyExists)?;
    }
    if state
        .service
        .user
        .get_user_by_gmail(&payload.email)
        .await
        .is_ok()
    {
        return Err(UserError::TryOtherMethod)?;
    }
    let now = state.env.now();
    let iat = now.timestamp();
    let exp = now
        .checked_add_signed(Duration::seconds(state.env.email_verify_exp_second))
        .unwrap()
        .timestamp();
    let passkey = state.env.generate_passkey().to_string();
    let verify_type = EmailVerifyType::VerifyEmail.to_string();
    let try_limit = state.env.email_verify_limit;

    let is_sent_passkey = if let Ok(temp_user) =
        state.service.user.tempuser_by_email(&payload.email).await
    {
        if iat < temp_user.iat.unwrap_or_default() + EMAIL_SEND_AGAIN_IN_SECONDS {
            return Err(UserError::CantSendEmail)?;
        }
        state
            .service
            .user
            .update_tempuser_with_email(
                &payload.email,
                &payload.name,
                &payload.password,
                &verify_type,
                &passkey,
                try_limit,
                iat,
                exp,
                now,
            )
            .await
            .map_err(|_| ApiError::DbError(DbError::Str("TempUser can't be updated".to_string())))?
    } else {
        state
            .service
            .user
            .create_tempuser_with_email(
                &payload.email,
                &payload.name,
                &payload.password,
                &verify_type,
                &passkey,
                try_limit,
                iat,
                exp,
                now,
            )
            .await
            .map_err(|_| ApiError::DbError(DbError::Str("TempUser can't be created".to_string())))?
    };

    if !send_auth_email(
        payload.email,
        passkey,
        EmailVerifyType::VerifyEmail,
        &state.ses_client,
    )
    .await
    {
        return Err(UserError::CantSendEmail)?;
    }
    Ok(Json(EmailVerificationResponse {
        is_sent: is_sent_passkey,
        iat,
        exp,
    }))
}

pub async fn resend_verification_email(
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ResendVerificationEmailRequest>,
) -> Result<Json<EmailVerificationResponse>, ApiError> {
    if !is_valid_email(&payload.email) {
        return Err(ApiError::UserError(UserError::Str(
            "The email is invalid".to_string(),
        )));
    }
    if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
        let now = state.env.now();
        let iat = now.timestamp();
        let exp = now
            .checked_add_signed(Duration::seconds(state.env.email_verify_exp_second))
            .unwrap()
            .timestamp();
        let passkey = state.env.generate_passkey().to_string();
        let verify_type = temp_user.verify_type.unwrap_or_default();
        let try_limit = temp_user.try_limit.unwrap_or_default();
        if iat < temp_user.iat.unwrap_or_default() + EMAIL_SEND_AGAIN_IN_SECONDS {
            return Err(UserError::CantSendEmail)?;
        }
        if !send_auth_email(
            payload.email.to_owned(),
            passkey.to_owned(),
            EmailVerifyType::VerifyEmail,
            &state.ses_client,
        )
        .await
        {
            return Err(UserError::CantSendEmail)?;
        }
        let is_sent_passkey = state
            .service
            .user
            .update_tempuser_with_email(
                &temp_user.email.unwrap_or_default(),
                &temp_user.name.unwrap_or_default(),
                &temp_user.password.unwrap_or_default(),
                &verify_type,
                &passkey,
                try_limit,
                iat,
                exp,
                now,
            )
            .await
            .map_err(|_| {
                ApiError::DbError(DbError::Str("TempUser can't be updated".to_string()))
            })?;
        return Ok(Json(EmailVerificationResponse {
            is_sent: is_sent_passkey,
            iat,
            exp,
        }));
    }
    Err(UserError::TempUserNotFound)?
}

pub async fn verify_email(
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<VerifyEmailRequest>,
) -> Result<Json<LoginAndRegisterResponse>, ApiError> {
    if !is_valid_email(&payload.email) {
        return Err(ApiError::UserError(UserError::Str(
            "The email is invalid".to_string(),
        )));
    }
    if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
        if temp_user.exp.unwrap_or(0) < state.env.now().timestamp() {
            return Err(UserError::ExpiredPasskey)?;
        }
        let verification_code = payload.verification_code.unwrap_or_default();
        if verification_code.is_empty()
            || !temp_user.passkey.unwrap_or_default().eq(&verification_code)
        {
            return Err(UserError::InvalidPasskey)?;
        }
        if state
            .service
            .user
            .get_user_by_email(&payload.email)
            .await
            .is_ok()
        {
            return Err(UserError::UserAlreadyExists)?;
        }
        if state
            .service
            .user
            .get_user_by_gmail(&payload.email)
            .await
            .is_ok()
        {
            return Err(UserError::UserAlreadyExists)?;
        }

        // Create the user with the stored information from temp_user
        let user = state
            .service
            .user
            .create_user_with_email(
                &temp_user.name.unwrap_or_default(),
                &temp_user.email.unwrap_or_default(),
                &bcrypt::hash(temp_user.password.unwrap_or_default(), 12).unwrap(),
            )
            .await?;

        // Verify the user's email
        state.service.user.verify_user_email(&payload.email).await?;

        // Delete the temp user after successful verification
        state
            .service
            .user
            .delete_tempuser_by_email(&payload.email)
            .await?;

        return Ok(Json(LoginAndRegisterResponse {
            user: UserReadDto::from(user.to_owned()),
            token: state
                .service
                .token
                .generate_token(user, UserRoleType::Member.to_string())?,
        }));
    }
    Err(UserError::TempUserNotFound)?
}

// pub async fn send_email_forgot_password(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserSendEmailForgotPwdRequest>,
// ) -> Result<Json<UserSendPasskeyResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::Str(
//             "The email is invalid".to_string(),
//         )));
//     }
//     state.service.user.find_by_email(&payload.email).await?;
//     let now = state.env.now();
//     let iat = now.timestamp();
//     let exp = now
//         .checked_add_signed(Duration::seconds(state.env.email_verify_exp_second))
//         .unwrap()
//         .timestamp();
//     let passkey = state.env.generate_passkey().to_string();
//     let verify_type = EmailVerifyType::ResetPassword.to_string();
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
//                     "",
//                     &verify_type,
//                     &passkey,
//                     try_limit,
//                     iat,
//                     exp,
//                     now,
//                 )
//                 .await
//                 .map_err(|_| {
//                     ApiError::DbError(DbError::Str(
//                         "TempUser can't be updated".to_string(),
//                     ))
//                 })?
//         } else {
//             state
//                 .service
//                 .user
//                 .create_tempuser_with_email(
//                     &payload.email,
//                     "",
//                     &verify_type,
//                     &passkey,
//                     try_limit,
//                     iat,
//                     exp,
//                     now,
//                 )
//                 .await
//                 .map_err(|_| {
//                     ApiError::DbError(DbError::Str(
//                         "TempUser can't be created".to_string(),
//                     ))
//                 })?
//         };

//     if !send_auth_email(
//         payload.email,
//         passkey,
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

// pub async fn resend_email_forgot_password(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserSendPasskeyAgainRequest>,
// ) -> Result<Json<UserSendPasskeyResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::Str(
//             "The email is invalid".to_string(),
//         )));
//     }
//     state.service.user.find_by_email(&payload.email).await?;
//     if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
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
//             payload.email.to_owned(),
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
//                 ApiError::DbError(DbError::Str(
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

// pub async fn verify_passkey_forgot_password(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserVerifyPasskeyRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::Str(
//             "The email is invalid".to_string(),
//         )));
//     }
//     if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
//         if temp_user.exp.unwrap_or(0) < state.env.now().timestamp() {
//             return Err(UserError::ExpiredPasskey)?;
//         }
//         let passkey = payload.passkey.unwrap_or_default();
//         if passkey.is_empty() || !temp_user.passkey.clone().unwrap_or_default().eq(&passkey) {
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
//                 ApiError::DbError(DbError::Str(
//                     "TempUser can't be updated".to_string(),
//                 ))
//             })?;
//         return Ok(Json(UserUpdateResponse { state: true }));
//     }
//     Err(UserError::TempUserNotFound)?
// }

// pub async fn reset_password(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UserResetPwdPwdRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if !is_valid_email(&payload.email) {
//         return Err(ApiError::UserError(UserError::Str(
//             "The email is invalid".to_string(),
//         )));
//     }
//     if let Ok(user) = state.service.user.find_by_email(&payload.email).await {
//         if let Ok(temp_user) = state.service.user.tempuser_by_email(&payload.email).await {
//             let passkey = payload.passkey;
//             if passkey.is_empty() || !temp_user.passkey.clone().unwrap_or_default().eq(&passkey) {
//                 return Err(UserError::InvalidPasskey)?;
//             }
//             if temp_user.exp.unwrap_or(0) < state.env.now().timestamp() {
//                 return Err(UserError::ExpiredPasskey)?;
//             }
//             let result = state
//                 .service
//                 .user
//                 .reset_password(user.id, payload.password)
//                 .await?;
//             let now = state.env.now();
//             let exp = now
//                 .checked_add_signed(Duration::seconds(1))
//                 .unwrap()
//                 .timestamp();
//             state
//                 .service
//                 .user
//                 .update_tempuser_with_email(
//                     &temp_user.email.unwrap_or_default(),
//                     &temp_user.password.unwrap_or_default(),
//                     &temp_user.verify_type.unwrap_or_default(),
//                     &temp_user.passkey.unwrap_or_default(),
//                     temp_user.try_limit.unwrap_or_default(),
//                     temp_user.iat.unwrap_or_default(),
//                     exp,
//                     now,
//                 )
//                 .await
//                 .map_err(|_| {
//                     ApiError::DbError(DbError::Str(
//                         "TempUser can't be updated".to_string(),
//                     ))
//                 })?;
//             return Ok(Json(result));
//         }
//         return Err(UserError::TempUserNotFound)?;
//     } else {
//         return Err(UserError::UserNotFound)?;
//     }
// }

// pub async fn add_users(
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AddUsersRequest>,
// ) -> Result<Json<AddUsersResponse>, ApiError> {
//     if payload.api_key.unwrap_or_default() != state.env.aws_access_key_id {
//         return Err(DbError::Str(
//             "API key is incorrect".to_string(),
//         ))?;
//     }
//     if let Some(gmail) = &payload.gmail {
//         if let Ok(user) = state.service.user.find_by_gmail(&gmail).await {
//             return Err(DbError::Str(format!(
//                 "Gmail is using now\nID: {}",
//                 user.id
//             )))?;
//         }
//     }
//     if let Some(principal) = &payload.principal {
//         if let Ok(user) = state.service.user.find_by_principal(&principal).await {
//             return Err(DbError::Str(format!(
//                 "Princiipal is using now\nID: {}",
//                 user.id
//             )))?;
//         }
//     }
//     let result = state
//         .service
//         .user
//         .create_user_with_data(
//             &payload.noble_id,
//             &payload.user_name,
//             &payload.first_name,
//             &payload.second_name,
//             &payload.gender,
//             &payload.degree,
//             &payload.country,
//             &payload.city,
//             &payload.avatar_url,
//             &payload.about_me,
//             &payload.principal,
//             &payload.gmail,
//             &payload.date_created,
//             &payload.date_updated,
//         )
//         .await?;
//     return Ok(Json(AddUsersResponse { id: result.id }));
// }
