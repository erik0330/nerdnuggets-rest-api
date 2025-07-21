use crate::{
    handler::auth_handler::{
        check_email, login_or_register_with_apple, login_or_register_with_google, login_with_email,
        register_with_email, resend_verification_email, verify_email,
    },
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login_with_email))
        .route("/auth/google", post(login_or_register_with_google))
        .route("/auth/apple", post(login_or_register_with_apple))
        .route("/auth/email/check", get(check_email))
        .route("/auth/register", post(register_with_email))
        .route("/auth/email/verify/resend", post(resend_verification_email))
        .route("/auth/email/verify", post(verify_email))
}
