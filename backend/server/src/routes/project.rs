use crate::{
    handler::project_handler::{create_project, get_project_by_id},
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/project/:id", get(get_project_by_id))
        .route("/project", post(create_project))
}
