use crate::{
    handler::{
        bounty_handler::{get_bids, get_bounties, get_bounty_by_id, get_similar_bounties},
        prediction_handler::{get_prediction_by_id, get_predictions, get_top_predictors},
        project_handler::{
            get_daos, get_project_by_id, get_project_counts_by_status, get_project_ids,
            get_projects, get_similar_projects,
        },
        user_handler::get_user_profile_by_username,
        util_handler::{get_categories, get_category_by_id, get_institutions},
    },
    state::AppState,
};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/profile/:username", get(get_user_profile_by_username))
        .route("/util/category", get(get_categories))
        .route("/util/category/:id", get(get_category_by_id))
        .route("/util/institutions", get(get_institutions))
        .route("/project", get(get_projects))
        .route("/project/:id", get(get_project_by_id))
        .route("/project/counts", get(get_project_counts_by_status))
        .route("/project/ids", get(get_project_ids))
        .route("/project/:id/similar", get(get_similar_projects))
        .route("/dao", get(get_daos))
        .route("/bounty", get(get_bounties))
        .route("/bounty/:id", get(get_bounty_by_id))
        .route("/bounty/:id/bid", get(get_bids))
        .route("/bounty/:id/similar", get(get_similar_bounties))
        .route("/prediction", get(get_predictions))
        .route("/prediction/:id", get(get_prediction_by_id))
        .route("/prediction/top", get(get_top_predictors))
}
