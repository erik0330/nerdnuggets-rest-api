use crate::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use types::dto::{ProjectUpdateStep1Request, ProjectUpdateStep2Request, ProjectUpdateStep3Request};
use types::error::{ApiError, ValidatedRequest};
use types::models::{ProjectInfo, User};

pub async fn get_project_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ProjectInfo>, ApiError> {
    let project = state.service.project.get_project_by_id(&id).await?;
    Ok(Json(project))
}

pub async fn create_project(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<ProjectInfo>, ApiError> {
    let project = state.service.project.create_project(user.id).await?;
    Ok(Json(project))
}

pub async fn update_project_step_1(
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ProjectUpdateStep1Request>,
) -> Result<Json<bool>, ApiError> {
    let res = state
        .service
        .project
        .update_project_step_1(&id, payload)
        .await?;
    Ok(Json(res))
}

pub async fn update_project_step_2(
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ProjectUpdateStep2Request>,
) -> Result<Json<bool>, ApiError> {
    let res = state
        .service
        .project
        .update_project_step_2(&id, payload)
        .await?;
    Ok(Json(res))
}

pub async fn update_project_step_3(
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ProjectUpdateStep3Request>,
) -> Result<Json<bool>, ApiError> {
    let res = state
        .service
        .project
        .update_project_step_3(&id, payload)
        .await?;
    Ok(Json(res))
}
