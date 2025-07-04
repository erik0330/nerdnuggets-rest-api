use crate::{
    handler::bounty_handler::{create_bounty, delete_bounty, submit_bid},
    state::AppState,
};
use axum::{
    routing::{delete, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/bounty", post(create_bounty))
        .route("/bounty", delete(delete_bounty))
        .route("/bounty/:id/bid", post(submit_bid))
}
