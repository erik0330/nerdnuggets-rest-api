use crate::response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Unable to create your account.")]
    CantCreateUser,
    #[error("Account not found.")]
    UserNotFound,
    #[error("Account already exists.")]
    UserAlreadyExists,
    #[error("Incorrect password. Please try again.")]
    InvalidPassword,
    #[error("Username already exists. Please choose a different one.")]
    UsernameAlreadyExists,
    #[error("Please verify your email to continue.")]
    EmailNotVerified,
    #[error("Temporary account not found.")]
    TempUserNotFound,
    #[error("Invalid code. Please check and try again.")]
    InvalidPasskey,
    #[error("Code expired. Please request a new one.")]
    ExpiredPasskey,
    #[error("Unable to send the email. Please try again.")]
    CantSendEmail,
    #[error("This email is already in use.")]
    EmailAlreadyUsed,
    #[error("Email does not match. Please check and try again.")]
    EmailNotMatch,
    #[error("You've already used this email to login with Google. Please use the 'Continue with Google'")]
    TryOtherMethod,
    #[error("This role is not allowed")]
    RoleNotAllowed,
    #[error("{0}")]
    SomethingWentWrong(String),
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserError::CantCreateUser => StatusCode::BAD_REQUEST,
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserAlreadyExists => StatusCode::BAD_REQUEST,
            UserError::InvalidPassword => StatusCode::BAD_REQUEST,
            UserError::UsernameAlreadyExists => StatusCode::BAD_REQUEST,
            UserError::EmailNotVerified => StatusCode::BAD_REQUEST,
            UserError::TempUserNotFound => StatusCode::NOT_FOUND,
            UserError::InvalidPasskey => StatusCode::BAD_REQUEST,
            UserError::ExpiredPasskey => StatusCode::BAD_REQUEST,
            UserError::CantSendEmail => StatusCode::BAD_REQUEST,
            UserError::EmailAlreadyUsed => StatusCode::BAD_REQUEST,
            UserError::EmailNotMatch => StatusCode::BAD_REQUEST,
            UserError::TryOtherMethod => StatusCode::BAD_REQUEST,
            UserError::RoleNotAllowed => StatusCode::BAD_REQUEST,
            UserError::SomethingWentWrong(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
