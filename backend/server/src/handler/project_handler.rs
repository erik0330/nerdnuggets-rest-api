use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use types::dto::{
    AssignEditorRequest, GetProjectsOption, MakeDecisionRequest, ProjectUpdateStep1Request,
    ProjectUpdateStep2Request, ProjectUpdateStep3Request, UpdateMilestoneRequest,
};
use types::error::{ApiError, UserError, ValidatedRequest};
use types::models::{ProjectIds, ProjectInfo, ProjectItemInfo, User};
use types::{FeedbackStatus, UserRoleType};

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

pub async fn submit_project(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state.service.project.submit_project(&id).await?;
    Ok(Json(res))
}

pub async fn get_project_ids(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectIds>>, ApiError> {
    let res = state.service.project.get_project_ids().await?;
    Ok(Json(res))
}

pub async fn get_projects(
    Extension(user): Extension<Option<User>>,
    Extension(role): Extension<Option<String>>,
    Query(opts): Query<GetProjectsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectItemInfo>>, ApiError> {
    let res = state
        .service
        .project
        .get_projects(
            opts.title,
            opts.status,
            opts.category_id,
            role,
            user.map(|u| u.id),
            opts.is_mine,
            opts.is_public,
            opts.offset,
            opts.limit,
        )
        .await?;
    Ok(Json(res))
}

pub async fn assign_editor(
    Extension(role): Extension<String>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<AssignEditorRequest>,
) -> Result<Json<bool>, ApiError> {
    if role != UserRoleType::Admin.to_string() {
        return Err(UserError::RoleNotAllowed)?;
    }
    Ok(Json(
        state
            .service
            .project
            .assign_editor(&id, payload.editor_id)
            .await?,
    ))
}

pub async fn make_decision(
    Extension(user): Extension<User>,
    Extension(role): Extension<String>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<MakeDecisionRequest>,
) -> Result<Json<bool>, ApiError> {
    if role == UserRoleType::Editor.to_string() {
        return Ok(Json(
            state
                .service
                .project
                .decide_editor(
                    &id,
                    user.id,
                    FeedbackStatus::from(payload.status),
                    payload.feedback,
                )
                .await?,
        ));
    } else if role == UserRoleType::Admin.to_string() {
        return Ok(Json(
            state
                .service
                .project
                .decide_admin(
                    &id,
                    FeedbackStatus::from(payload.status),
                    payload.feedback,
                    payload.to_dao.unwrap_or_default(),
                )
                .await?,
        ));
    }
    Err(UserError::RoleNotAllowed)?
}

pub async fn update_milestone(
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<UpdateMilestoneRequest>,
) -> Result<Json<bool>, ApiError> {
    Ok(Json(
        state.service.project.update_milestone(&id, payload).await?,
    ))
}
