use crate::state::AppState;
use axum::{
    extract::State,
    headers::{
        authorization::{Authorization, Bearer},
        Header,
    },
    http::{self, Request},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::errors::ErrorKind;
use types::{
    error::{ApiError, TokenError, UserError},
    models::User,
    UserRoleType,
};

pub async fn auth<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(header_name, header_value)| {
            if header_name == http::header::AUTHORIZATION {
                return Some(header_value);
            }
            None
        });

    let header: Authorization<Bearer> =
        Authorization::decode(&mut headers).map_err(|_| TokenError::MissingToken)?;

    let token = header.token();
    match state.service.token.retrieve_token_claims(token) {
        Ok(token_data) => {
            let user = state
                .service
                .user
                .find_by_user_id(token_data.claims.sub)
                .await;
            match user {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    req.extensions_mut().insert(token_data.claims.role);
                    Ok(next.run(req).await)
                }
                Err(_) => Err(UserError::UserNotFound)?,
            }
        }
        Err(err) => {
            return match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired)?,
                _ => Err(TokenError::InvalidToken(token.parse().unwrap_or_default()))?,
            };
        }
    }
}

pub async fn public<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(header_name, header_value)| {
            if header_name == http::header::AUTHORIZATION {
                return Some(header_value);
            }
            None
        });

    let header = match Authorization::decode(&mut headers) {
        Ok(head) => head,
        Err(_) => {
            let user: Option<User> = None;
            let role: Option<UserRoleType> = None;
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(role);
            return Ok(next.run(req).await);
        }
    };

    let token = header.token();
    match state.service.token.retrieve_token_claims(token) {
        Ok(token_data) => {
            let user = state
                .service
                .user
                .find_by_user_id(token_data.claims.sub)
                .await;
            match user {
                Ok(user) => {
                    req.extensions_mut().insert(Some(user));
                    req.extensions_mut().insert(Some(token_data.claims.role));
                    Ok(next.run(req).await)
                }
                Err(_) => Err(UserError::UserNotFound)?,
            }
        }
        Err(err) => {
            return match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired)?,
                _ => Err(TokenError::InvalidToken(token.parse().unwrap_or_default()))?,
            };
        }
    }
}
