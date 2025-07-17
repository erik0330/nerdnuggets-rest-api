use crate::{handler::notification_handler::*, state::AppState};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_notifications))
        .route("/unread", get(get_unread_notifications))
        .route("/count", get(get_notification_count))
        .route("/", post(create_notification))
        .route("/read-all", put(mark_all_notifications_as_read))
        .route("/:id/read", put(mark_notification_as_read))
        .route("/:id", delete(delete_notification))
}
