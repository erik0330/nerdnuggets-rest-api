use crate::{
    handler::bounty_handler::{create_bounty, get_bounty_by_id},
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/bounty/:id", get(get_bounty_by_id))
        .route("/bounty", post(create_bounty))
}
