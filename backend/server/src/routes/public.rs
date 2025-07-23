use crate::{
    handler::{
        bounty_handler::{get_bounties, get_bounty_by_id, get_similar_bounties},
        prediction_handler::{get_prediction_by_id, get_predictions},
        project_handler::{get_daos, get_project_ids, get_projects, get_similar_projects},
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
        .route("/project/ids", get(get_project_ids))
        .route("/project/:id/similar", get(get_similar_projects))
        .route("/dao", get(get_daos))
        .route("/bounty", get(get_bounties))
        .route("/bounty/:id", get(get_bounty_by_id))
        .route("/bounty/:id/similar", get(get_similar_bounties))
        .route("/prediction", get(get_predictions))
        .route("/prediction/:id", get(get_prediction_by_id))
}
