use crate::{
    handler::project_handler::{create_project, get_project_by_id, update_project_step_1},
    state::AppState,
};
use axum::{
    routing::{get, patch, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/project/:id", get(get_project_by_id))
        .route("/project", post(create_project))
        .route("/project/:id/1", patch(update_project_step_1))
}
