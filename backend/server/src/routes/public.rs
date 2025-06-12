use crate::{
    handler::util_handler::{get_categories, get_category_by_id},
    state::AppState,
};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        // .route("/user/profile", get(get_profile))
        // .route("/user/members", get(get_members))
        .route("/util/category", get(get_categories))
        .route("/util/category/:id", get(get_category_by_id))
}
