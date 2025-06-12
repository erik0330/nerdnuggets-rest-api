use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
    // .route(
    //     "/admin/dashboard/tabs",
    //     get(get_user_counts_on_dashboard_tab),
    // )
    // .route("/admin/remove_avatar", post(remove_avatar))
    // .route("/admin/add_admin", post(add_admin))
    // .route("/admin/remove_admin", post(remove_admin))
    // .route("/admin/suspend", post(suspend_user))
    // .route("/admin/approve_role", post(approve_user_role))
    // .route("/admin/update_userinfo", post(update_user_info))
    // .route("/admin/reports/members", get(get_reported_members))
    // .route("/admin/user/unreport", post(unreport_user))
    // .route("/admin/user/delete", post(delete_user))
    // .route("/admin/category/add", post(add_categories))
    // .route("/admin/category/update", post(update_category))
    // .route("/admin/category/delete", post(delete_category))
    // .route("/admin/degree/update/:degree_id", post(update_degree))
    // .route("/admin/degree/delete/:degree_id", post(delete_degree))
}
