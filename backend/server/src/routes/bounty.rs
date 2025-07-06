use crate::{
    handler::bounty_handler::{
        create_bounty, delete_bounty, get_bounty_comments, submit_bid, submit_bounty_comment,
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/bounty", post(create_bounty))
        .route("/bounty/:id", delete(delete_bounty))
        .route("/bounty/:id/bid", post(submit_bid))
        .route("/bounty/:id/comment", get(get_bounty_comments))
        .route("/bounty/:id/comment", post(submit_bounty_comment))
}
