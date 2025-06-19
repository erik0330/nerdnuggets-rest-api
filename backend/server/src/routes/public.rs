use crate::{
    handler::{
        project_handler::get_projects,
        util_handler::{get_categories, get_category_by_id},
    },
    state::AppState,
};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/util/category", get(get_categories))
        .route("/util/category/:id", get(get_category_by_id))
        .route("/project", get(get_projects))
}
