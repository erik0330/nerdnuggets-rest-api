use crate::{
    handler::util_handler::{
        get_city, get_country, get_institution_detail, get_institutions, upload_add, upload_remove,
    },
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/util/country", get(get_country))
        .route("/util/city", get(get_city))
        .route("/util/institutions", get(get_institutions))
        .route("/util/institution/detail", get(get_institution_detail))
        // .route("/util/degree", get(get_degree))
        // .route("/util/degree/update", post(update_degree))
        // .route("/util/hashtags", get(get_hashtags))
        // .route("/util/employments", get(get_employments))
        // .route("/util/wallpapers", get(get_wallpapers))
        .route("/util/upload/add/:folder", post(upload_add))
        .route("/util/remove", post(upload_remove))
}
