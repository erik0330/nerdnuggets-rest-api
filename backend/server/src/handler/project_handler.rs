use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use third_party_api::arweave::upload_project_submission;
use types::dto::{
    AdminProjectDashboardCounts, AssignEditorRequest, GetDaosOption, GetProjectCommentsOption,
    GetProjectsOption, GetSimilarProjectsOption, MakeDecisionRequest, ProjectCountsResponse,
    ProjectUpdateStep1Request, ProjectUpdateStep2Request, ProjectUpdateStep3Request,
    SubmitDaoVoteRequest, SubmitProjectCommentRequest, UpdateMilestoneRequest,
};
use types::error::{ApiError, UserError, ValidatedRequest};
use types::models::{
    DaoInfo, DaoVote, Milestone, ProjectCommentInfo, ProjectIds, ProjectInfo, ProjectItemInfo, User,
};
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

pub async fn delete_project(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state.service.project.delete_project(&id, user.id).await?;
    Ok(Json(res))
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
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let user = state.service.user.get_user_by_id(user.id).await?;
    if user.wallet_address.filter(|w| !w.is_empty()).is_none() {
        return Err(UserError::Str("Wallet address is not set".to_string()).into());
    }
    let res = state.service.project.submit_project(&id).await?;

    let project_info = state
        .service
        .project
        .get_project_by_id_without_increment(&id)
        .await?;
    if res {
        // Upload project submission metadata to Arweave
        let project_data = serde_json::json!(project_info);

        match upload_project_submission(
            &project_info.id.to_string(),
            &project_info.nerd_id,
            &user.id.to_string(),
            &project_data,
        )
        .await
        {
            Ok(arweave_id) => {
                state
                    .service
                    .project
                    .update_project_arweave_tx_id(project_info.id, &arweave_id)
                    .await?;
            }
            Err(_e) => {}
        }
    }

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
                    &state.evm,
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

pub async fn get_milestones(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Milestone>>, ApiError> {
    Ok(Json(state.service.project.get_milestones(&id).await?))
}

pub async fn get_project_comments(
    Path(id): Path<String>,
    Query(opts): Query<GetProjectCommentsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectCommentInfo>>, ApiError> {
    Ok(Json(
        state
            .service
            .project
            .get_project_comments(&id, opts.offset, opts.limit)
            .await?,
    ))
}

pub async fn submit_project_comment(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SubmitProjectCommentRequest>,
) -> Result<Json<bool>, ApiError> {
    Ok(Json(
        state
            .service
            .project
            .submit_project_comment(&id, user.id, &payload.comment)
            .await?,
    ))
}

pub async fn get_daos(
    Extension(user): Extension<Option<User>>,
    Query(opts): Query<GetDaosOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<DaoInfo>>, ApiError> {
    Ok(Json(
        state
            .service
            .project
            .get_daos(
                opts.title,
                opts.status,
                user.map(|u| u.id),
                opts.is_mine,
                opts.offset,
                opts.limit,
            )
            .await?,
    ))
}

pub async fn get_dao_by_id(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DaoInfo>, ApiError> {
    Ok(Json(
        state.service.project.get_dao_by_id(&id, user.id).await?,
    ))
}

pub async fn get_my_dao_vote(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Option<DaoVote>>, ApiError> {
    Ok(Json(
        state.service.project.get_my_dao_vote(&id, user.id).await?,
    ))
}

pub async fn submit_dao_vote(
    Extension(_user): Extension<User>,
    Path(_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SubmitDaoVoteRequest>,
) -> Result<Json<bool>, ApiError> {
    let proposal_id: i64 = payload.proposal_id.parse().unwrap_or_default();
    let weight: u128 = payload.weight.parse().unwrap_or_default();
    let res = state
        .service
        .project
        .submit_dao_vote(proposal_id, &payload.wallet, payload.support, weight)
        .await?;
    Ok(Json(res))
}

pub async fn get_similar_projects(
    Path(id): Path<String>,
    Query(opts): Query<GetSimilarProjectsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectItemInfo>>, ApiError> {
    let similar_projects = state
        .service
        .project
        .get_similar_projects(&id, opts.limit)
        .await?;
    Ok(Json(similar_projects))
}

pub async fn get_project_counts_by_status(
    Extension(user): Extension<Option<User>>,
    State(state): State<AppState>,
) -> Result<Json<ProjectCountsResponse>, ApiError> {
    let counts = state
        .service
        .project
        .get_project_counts_by_status_for_user(user.map(|u| u.id))
        .await?;
    Ok(Json(counts))
}

pub async fn get_admin_project_dashboard_counts(
    Extension(role): Extension<String>,
    State(state): State<AppState>,
) -> Result<Json<AdminProjectDashboardCounts>, ApiError> {
    if role != UserRoleType::Admin.to_string() {
        return Err(UserError::RoleNotAllowed)?;
    }

    let counts = state
        .service
        .project
        .get_admin_project_dashboard_counts()
        .await?;
    Ok(Json(counts))
}
