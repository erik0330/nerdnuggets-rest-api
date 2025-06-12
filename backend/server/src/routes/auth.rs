use crate::{
    handler::auth_handler::{
        check_email, login_or_register_with_google, login_with_email, register_with_email,
    },
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login_with_email", post(login_with_email))
        .route(
            "/auth/login_with_google",
            post(login_or_register_with_google),
        )
        .route("/auth/check_email", get(check_email))
        .route("/auth/register_with_email", post(register_with_email))
}
