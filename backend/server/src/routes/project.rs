use crate::{
    handler::{
        prediction_handler::get_my_prediction_stats,
        project_handler::{
            approve_reject_milestone, assign_editor, create_project, delete_project,
            get_admin_dashboard_counts, get_admin_project_dashboard_counts,
            get_editor_dashboard_counts, get_milestone_submissions, get_milestones,
            get_my_dao_vote, get_nomination_by_id, get_nomination_by_identifier,
            get_nomination_statistics, get_nominations, get_research_projects_dashboard,
            get_user_dao_votes, get_user_dao_voting_stats, make_decision, submit_dao_vote,
            submit_project, submit_project_comment, update_milestone, update_project_step_1,
            update_project_step_2, update_project_step_3,
        },
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/project", post(create_project))
        .route("/project/:id", delete(delete_project))
        .route("/project/:id/1", patch(update_project_step_1))
        .route("/project/:id/2", patch(update_project_step_2))
        .route("/project/:id/3", patch(update_project_step_3))
        .route("/project/:id/submit", post(submit_project))
        .route("/project/:id/editor", post(assign_editor))
        .route("/project/:id/decide", patch(make_decision))
        .route("/milestone/:id", patch(update_milestone))
        .route("/milestone/:id/approve", patch(approve_reject_milestone))
        .route("/milestone/submissions", get(get_milestone_submissions))
        .route("/project/:id/milestone", get(get_milestones))
        .route("/project/:id/comment", post(submit_project_comment))
        .route("/dao/:id/vote", post(submit_dao_vote))
        .route("/dao/:id/vote", get(get_my_dao_vote))
        .route("/dao/stats", get(get_user_dao_voting_stats))
        .route("/dao/votes", get(get_user_dao_votes))
        .route("/prediction/stats", get(get_my_prediction_stats))
        .route(
            "/project/admin/counts",
            get(get_admin_project_dashboard_counts),
        )
        .route("/project/admin/dashboard", get(get_admin_dashboard_counts))
        .route("/project/editor/counts", get(get_editor_dashboard_counts))
        .route(
            "/project/research/dashboard",
            get(get_research_projects_dashboard),
        )
        .route("/nomination", get(get_nominations))
        .route("/nomination/stats", get(get_nomination_statistics))
        .route(
            "/nomination/identifier/:identifier",
            get(get_nomination_by_identifier),
        )
        .route("/nomination/:id", get(get_nomination_by_id))
}
