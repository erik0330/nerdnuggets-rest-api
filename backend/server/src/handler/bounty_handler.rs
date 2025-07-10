use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};

use types::dto::{
    BountyCreateRequest, BountyUpdateRequest, GetBountysOption, OffsetAndLimitOption,
    ReviewBountyRequest, SubmitBidRequest, SubmitBountyCommentRequest,
};
use types::error::{ApiError, DbError, ValidatedRequest};
use types::models::{BidInfo, BountyCommentInfo, BountyInfo, User};
use types::UserRoleType;

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
    let bounty = state
        .service
        .bounty
        .create_bounty(user.id, payload.clone())
        .await?;
    // let tags = vec![
    //     ("Content-Type".to_string(), "application/json".to_string()),
    //     ("App-Name".to_string(), "NerdNuggets".to_string()),
    //     ("Type".to_string(), "Bounty".to_string()),
    // ];
    // if let Ok(link) =
    //     upload_metadata_to_arweave(&tags, &serde_json::to_string(&payload).unwrap()).await
    // {
    //     println!("link: {}", link);
    // } else {
    //     println!("link error");
    // }
    Ok(Json(bounty))
}

pub async fn update_bounty(
    Extension(role): Extension<String>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<BountyUpdateRequest>,
) -> Result<Json<bool>, ApiError> {
    if role != UserRoleType::Admin.to_string() {
        return Err(DbError::Str("You are not an admin.".to_string()).into());
    }
    let res = state.service.bounty.update_bounty(&id, payload).await?;
    Ok(Json(res))
}

pub async fn delete_bounty(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state.service.bounty.delete_bounty(&id, user.id).await?;
    Ok(Json(res))
}

pub async fn get_bounties(
    Extension(user): Extension<Option<User>>,
    Extension(role): Extension<Option<String>>,
    Query(opts): Query<GetBountysOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BountyInfo>>, ApiError> {
    let res = state
        .service
        .bounty
        .get_bounties(
            opts.title,
            opts.status,
            opts.category_id,
            opts.difficulty,
            role,
            user.map(|u| u.id),
            opts.is_mine,
            opts.offset,
            opts.limit,
        )
        .await?;
    Ok(Json(res))
}

pub async fn get_bids(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Query(opts): Query<OffsetAndLimitOption>,
) -> Result<Json<Vec<BidInfo>>, ApiError> {
    let bids = state
        .service
        .bounty
        .get_bids(&id, opts.offset, opts.limit)
        .await?;
    Ok(Json(bids))
}

pub async fn submit_bid(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SubmitBidRequest>,
) -> Result<Json<BidInfo>, ApiError> {
    let bid = state.service.bounty.submit_bid(&id, user, payload).await?;
    Ok(Json(bid))
}

pub async fn get_bounty_comments(
    Path(id): Path<String>,
    Query(opts): Query<OffsetAndLimitOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BountyCommentInfo>>, ApiError> {
    Ok(Json(
        state
            .service
            .bounty
            .get_bounty_comments(&id, opts.offset, opts.limit)
            .await?,
    ))
}

pub async fn submit_bounty_comment(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SubmitBountyCommentRequest>,
) -> Result<Json<bool>, ApiError> {
    Ok(Json(
        state
            .service
            .bounty
            .submit_bounty_comment(&id, user.id, &payload.comment)
            .await?,
    ))
}

pub async fn review_bounty(
    Extension(role): Extension<String>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ReviewBountyRequest>,
) -> Result<Json<bool>, ApiError> {
    if role != UserRoleType::Admin.to_string() {
        return Err(DbError::Str("You are not an admin.".to_string()).into());
    }
    let res = state
        .service
        .bounty
        .review_bounty(&id, payload.status, payload.admin_notes)
        .await?;
    Ok(Json(res))
}
