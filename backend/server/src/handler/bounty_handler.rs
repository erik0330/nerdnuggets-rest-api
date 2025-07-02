use crate::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};

use types::dto::BountyCreateRequest;
use types::error::{ApiError, ValidatedRequest};
use types::models::{BountyInfo, User};

pub async fn get_bounty_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BountyInfo>, ApiError> {
    let bounty = state.service.bounty.get_bounty_by_id(&id).await?;
    Ok(Json(bounty))
}

pub async fn create_bounty(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<BountyCreateRequest>,
) -> Result<Json<BountyInfo>, ApiError> {
    let bounty = state.service.bounty.create_bounty(user.id, payload).await?;
    Ok(Json(bounty))
}

// pub async fn update_bounty_step_3(
//     Path(id): Path<String>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<BountyUpdateStep3Request>,
// ) -> Result<Json<bool>, ApiError> {
//     let res = state
//         .service
//         .bounty
//         .update_bounty_step_3(&id, payload)
//         .await?;
//     Ok(Json(res))
// }

// pub async fn submit_bounty(
//     Path(id): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<bool>, ApiError> {
//     let res = state.service.bounty.submit_bounty(&id).await?;
//     Ok(Json(res))
// }

// pub async fn get_bounty_ids(
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<BountyIds>>, ApiError> {
//     let res = state.service.bounty.get_bounty_ids().await?;
//     Ok(Json(res))
// }

// pub async fn get_bountys(
//     Extension(user): Extension<Option<User>>,
//     Extension(role): Extension<Option<String>>,
//     Query(opts): Query<GetBountysOption>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<BountyItemInfo>>, ApiError> {
//     let res = state
//         .service
//         .bounty
//         .get_bountys(
//             opts.title,
//             opts.status,
//             opts.category_id,
//             role,
//             user.map(|u| u.id),
//             opts.is_mine,
//             opts.is_public,
//             opts.offset,
//             opts.limit,
//         )
//         .await?;
//     Ok(Json(res))
// }

// pub async fn assign_editor(
//     Extension(role): Extension<String>,
//     Path(id): Path<String>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AssignEditorRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     if role != UserRoleType::Admin.to_string() {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     Ok(Json(
//         state
//             .service
//             .bounty
//             .assign_editor(&id, payload.editor_id)
//             .await?,
//     ))
// }

// pub async fn make_decision(
//     Extension(user): Extension<User>,
//     Extension(role): Extension<String>,
//     Path(id): Path<String>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<MakeDecisionRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     if role == UserRoleType::Editor.to_string() {
//         return Ok(Json(
//             state
//                 .service
//                 .bounty
//                 .decide_editor(
//                     &id,
//                     user.id,
//                     FeedbackStatus::from(payload.status),
//                     payload.feedback,
//                 )
//                 .await?,
//         ));
//     } else if role == UserRoleType::Admin.to_string() {
//         return Ok(Json(
//             state
//                 .service
//                 .bounty
//                 .decide_admin(
//                     &id,
//                     FeedbackStatus::from(payload.status),
//                     payload.feedback,
//                     payload.to_dao.unwrap_or_default(),
//                 )
//                 .await?,
//         ));
//     }
//     Err(UserError::RoleNotAllowed)?
// }

// pub async fn update_milestone(
//     Path(id): Path<String>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<UpdateMilestoneRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     Ok(Json(
//         state.service.bounty.update_milestone(&id, payload).await?,
//     ))
// }

// pub async fn get_milestones(
//     Path(id): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<Milestone>>, ApiError> {
//     Ok(Json(state.service.bounty.get_milestones(&id).await?))
// }

// pub async fn get_bounty_comments(
//     Path(id): Path<String>,
//     Query(opts): Query<GetBountyCommentsOption>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<BountyCommentInfo>>, ApiError> {
//     Ok(Json(
//         state
//             .service
//             .bounty
//             .get_bounty_comments(&id, opts.offset, opts.limit)
//             .await?,
//     ))
// }

// pub async fn submit_bounty_comment(
//     Extension(user): Extension<User>,
//     Path(id): Path<String>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<SubmitBountyCommentRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     Ok(Json(
//         state
//             .service
//             .bounty
//             .submit_bounty_comment(&id, user.id, &payload.comment)
//             .await?,
//     ))
// }

// pub async fn get_daos(
//     Extension(user): Extension<Option<User>>,
//     Query(opts): Query<GetDaosOption>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<Dao>>, ApiError> {
//     Ok(Json(
//         state
//             .service
//             .bounty
//             .get_daos(
//                 opts.title,
//                 opts.status,
//                 user.map(|u| u.id),
//                 opts.is_mine,
//                 opts.offset,
//                 opts.limit,
//             )
//             .await?,
//     ))
// }

// pub async fn get_dao_by_id(
//     Path(id): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<Dao>, ApiError> {
//     Ok(Json(state.service.bounty.get_dao_by_id(&id).await?))
// }

// pub async fn get_my_dao_vote(
//     Extension(user): Extension<User>,
//     Path(id): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<Option<DaoVote>>, ApiError> {
//     Ok(Json(
//         state.service.bounty.get_my_dao_vote(&id, user.id).await?,
//     ))
// }

// pub async fn submit_dao_vote(
//     Extension(user): Extension<User>,
//     Path(id): Path<String>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<SubmitVoteRequest>,
// ) -> Result<Json<bool>, ApiError> {
//     Ok(Json(
//         state
//             .service
//             .bounty
//             .submit_dao_vote(&id, user.id, payload.status, payload.comment)
//             .await?,
//     ))
// }
