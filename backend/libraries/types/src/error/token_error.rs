use crate::response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("The token is invalid. Please try again.")]
    InvalidToken(String),
    #[error("Your token has expired. Please log in again.")]
    TokenExpired,
    #[error("Token is missing. Please provide a valid token.")]
    MissingToken,
    #[error("Authentication is expired. Please try again")]
    AuthExpired,
    #[error("Token error: {0}")]
    TokenCreationError(String),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let status_code = match self {
            TokenError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            TokenError::TokenExpired => StatusCode::UNAUTHORIZED,
            TokenError::MissingToken => StatusCode::UNAUTHORIZED,
            TokenError::AuthExpired => StatusCode::UNAUTHORIZED,
            TokenError::TokenCreationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
