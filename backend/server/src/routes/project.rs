use crate::{
    handler::project_handler::{
        assign_editor, create_project, decide_review, get_project_by_id, publish, start_dao,
        submit_project, update_project_step_1, update_project_step_2, update_project_step_3,
    },
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
        .route("/project/:id/2", patch(update_project_step_2))
        .route("/project/:id/3", patch(update_project_step_3))
        .route("/project/:id/submit", post(submit_project))
        .route("/project/:id/editor", post(assign_editor))
        .route("/project/:id/decide", patch(decide_review))
        .route("/project/:id/dao", patch(start_dao))
        .route("/project/:id/publish", patch(publish))
}
