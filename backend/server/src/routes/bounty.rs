use crate::{
    handler::bounty_handler::{create_bounty, submit_bid},
    state::AppState,
};
use axum::{routing::post, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/bounty", post(create_bounty))
        .route("/bounty/:id/bid", post(submit_bid))
}
