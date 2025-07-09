use crate::{
    handler::project_handler::{
        assign_editor, create_project, delete_project, get_dao_by_id, get_milestones,
        get_my_dao_vote, get_project_by_id, get_project_comments, make_decision, submit_dao_vote,
        submit_project, submit_project_comment, update_milestone, update_project_step_1,
        update_project_step_2, update_project_step_3,
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/project/:id", get(get_project_by_id))
        .route("/project", post(create_project))
        .route("/project/:id", delete(delete_project))
        .route("/project/:id/1", patch(update_project_step_1))
        .route("/project/:id/2", patch(update_project_step_2))
        .route("/project/:id/3", patch(update_project_step_3))
        .route("/project/:id/submit", post(submit_project))
        .route("/project/:id/editor", post(assign_editor))
        .route("/project/:id/decide", patch(make_decision))
        .route("/milestone/:id", patch(update_milestone))
        .route("/project/:id/milestone", get(get_milestones))
        .route("/project/:id/comment", get(get_project_comments))
        .route("/project/:id/comment", post(submit_project_comment))
        .route("/dao/:id", get(get_dao_by_id))
        .route("/dao/:id/vote", get(get_my_dao_vote))
        .route("/dao/:id/vote", post(submit_dao_vote))
}
