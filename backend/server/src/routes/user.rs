use crate::{
    handler::user_handler::{
        change_role, get_editors, get_my_activities, get_user, update_user_onboarding,
    },
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user", get(get_user))
        .route("/user/editors", get(get_editors))
        .route("/user/onboarding/:id", post(update_user_onboarding))
        .route("/user/relogin", post(change_role))
        .route("/user/activities/me", get(get_my_activities))
    // .route("/user/check_username", get(check_username))
    // .route("/user/update_username", post(update_username))
    // .route("/user/update_profile", post(update_profile))
    // .route("/user/upload_avatar", post(upload_avatar))
    // .route("/user/remove_avatar", post(remove_avatar))
    // .route("/user/wallpaper/upload", post(upload_wallpaper))
    // .route("/user/wallpaper/update", post(update_wallpaper))
    // .route("/user/wallpaper/remove", post(remove_wallpaper))
    // .route("/user/setting", get(get_settings))
    // .route("/user/setting_profile", post(update_setting_profile))
    // .route(
    //     "/user/nobleblocks_role/update",
    //     post(update_nobleblocks_role),
    // )
    // .route("/user/roles", get(get_roles))
    // .route("/user/roles", post(update_roles))
    // .route("/user/publishing", get(get_publishing))
    // .route("/user/publishing", post(update_publishing))
    // .route("/user/notification", get(get_notification))
    // .route("/user/notification", post(update_notification))
    // .route("/user/social", get(get_social))
    // .route("/user/connect/telegram", post(connect_telegram))
    // .route("/user/connect/twitter", post(connect_twitter))
    // .route("/user/connect/google", post(connect_google))
    // .route("/user/connect/website", post(connect_website))
    // .route("/user/connect/linkedin", post(connect_linkedin))
    // .route("/user/connect/orc_id", post(connect_orc_id))
    // .route("/user/connect/google_scholar", post(connect_google_scholar))
    // .route("/user/email", post(update_email))
    // .route("/user/email/resend", post(resend_email_verify_code))
    // .route("/user/email/check", post(verify_passkey_change_email))
    // .route("/user/privacy", get(get_privacy))
    // .route(
    //     "/user/password_reset",
    //     post(send_email_verify_code_to_reset_password),
    // )
    // .route(
    //     "/user/password_reset/resend",
    //     post(resend_email_verify_code_to_reset_password),
    // )
    // .route(
    //     "/user/password_reset/check",
    //     post(verify_passkey_reset_password),
    // )
    // .route("/user/password_reset/save", post(update_password))
    // .route("/user/active", post(update_is_active))
    // .route("/user/follow", post(follow_user))
    // .route("/user/unfollow", post(unfollow_user))
    // .route("/user/report", post(report_user))
    // .route("/user/block", post(block_user))
    // .route("/user/suggested", get(get_suggested_user))
    // .route("/user/ids", get(get_users_by_ids))
    // .route("/user/subscription", post(add_subscription))
    // .route("/user/notifications", get(get_notifications))
    // .route("/user/mark_as_read", post(mark_notification_as_read))
    // .route("/user/notification", patch(read_notification))
}
