use crate::{
    handler::user_handler::{
        change_role, get_editors, get_my_activities, get_user, get_user_settings,
        update_notification_settings, update_preferences_settings, update_privacy_settings,
        update_profile_settings, update_user_onboarding, update_wallet_settings,
    },
    state::AppState,
};
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user", get(get_user))
        .route("/user/editors", get(get_editors))
        .route("/user/onboarding/:id", post(update_user_onboarding))
        .route("/user/relogin", post(change_role))
        .route("/user/activities/me", get(get_my_activities))
        // User Settings Routes
        .route("/user/settings", get(get_user_settings))
        .route("/user/settings/profile", put(update_profile_settings))
        .route(
            "/user/settings/notifications",
            put(update_notification_settings),
        )
        .route("/user/settings/privacy", put(update_privacy_settings))
        .route("/user/settings/wallet", put(update_wallet_settings))
        .route(
            "/user/settings/preferences",
            put(update_preferences_settings),
        )
}
