use crate::{handler::notification_handler::*, state::AppState};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/notification", get(get_notifications))
        .route("/notification/unread", get(get_unread_notifications))
        .route("/notification/count", get(get_notification_count))
        .route("/notification", post(create_notification))
        .route(
            "/notification/read-all",
            put(mark_all_notifications_as_read),
        )
        .route("/notification/:id/read", put(mark_notification_as_read))
        .route("/notification/:id", delete(delete_notification))
}
