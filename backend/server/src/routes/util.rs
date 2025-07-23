use crate::{
    handler::util_handler::{
        extract_project_info, get_city, get_country, get_institution_detail, get_institutions,
        remove_file_from_s3, upload_file_to_s3,
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/util/country", get(get_country))
        .route("/util/city", get(get_city))
        .route("/util/institutions", get(get_institutions))
        .route("/util/institution/detail", get(get_institution_detail))
        .route("/util/file/:folder", post(upload_file_to_s3))
        .route("/util/file", delete(remove_file_from_s3))
        .route("/util/extract-project-info", get(extract_project_info))
}
