use crate::{
    handler::bounty_handler::{
        create_bounty, delete_bounty, get_bids, get_bounty_comments, review_bounty, submit_bid,
        submit_bounty_comment,
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
        .route("/bounty/:id/bid", get(get_bids))
        .route("/bounty/:id/bid", post(submit_bid))
        // .route("/bid/:id/win", put(select_bid_winner))
        .route("/bounty/:id/comment", get(get_bounty_comments))
        .route("/bounty/:id/comment", post(submit_bounty_comment))
        .route("/bounty/:id/review", post(review_bounty))
}
