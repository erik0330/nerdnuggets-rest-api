use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};

use types::dto::{
    BountyChatListResponse, BountyCreateRequest, BountyUpdateRequest, GetBidsOption,
    GetBountyChatNumbersResponse, GetBountyChatsOption, GetBountysOption, GetMyBidsOption,
    GetMyBountyStatsResponse, GetSimilarBountiesOption, OffsetAndLimitOption, ReviewBountyRequest,
    SendBountyChatRequest, SubmitBidRequest, SubmitBountyCommentRequest,
};
use types::error::{ApiError, DbError, ValidatedRequest};
use types::models::{BidInfo, BountyChatInfo, BountyCommentInfo, BountyInfo, User};
use types::UserRoleType;
use utils::commons::uuid_from_str;

pub async fn get_bounty_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BountyInfo>, ApiError> {
    let bounty = state.service.bounty.get_bounty_info_by_id(&id).await?;
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
    Query(opts): Query<GetBidsOption>,
) -> Result<Json<Vec<BidInfo>>, ApiError> {
    let bids = state
        .service
        .bounty
        .get_bids(&id, opts.status, opts.offset, opts.limit)
        .await?;
    Ok(Json(bids))
}

pub async fn get_my_bids(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Query(opts): Query<GetMyBidsOption>,
) -> Result<Json<Vec<BidInfo>>, ApiError> {
    let bids = state
        .service
        .bounty
        .get_my_bids(user, opts.status, opts.offset, opts.limit)
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

pub async fn select_as_winner(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state.service.bounty.select_as_winner(&id).await?;
    Ok(Json(res))
}

pub async fn reject_bid(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state.service.bounty.reject_bid(&id).await?;
    Ok(Json(res))
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

pub async fn get_my_bounty_stats(
    Extension(_user): Extension<User>,
    State(_state): State<AppState>,
) -> Result<Json<GetMyBountyStatsResponse>, ApiError> {
    Ok(Json(GetMyBountyStatsResponse {
        total_earned: 0,
        completed: 0,
        in_progress: 0,
        success_rate: 0,
    }))
}

pub async fn get_bounty_chats(
    Path(id): Path<String>,
    Query(opts): Query<GetBountyChatsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BountyChatInfo>>, ApiError> {
    let chats = state
        .service
        .bounty
        .get_bounty_chats(&id, &opts.chat_number, opts.offset, opts.limit)
        .await?;
    Ok(Json(chats))
}

pub async fn send_bounty_chat(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SendBountyChatRequest>,
) -> Result<Json<bool>, ApiError> {
    let res = state
        .service
        .bounty
        .send_bounty_chat(
            &id,
            user.id,
            &payload.message,
            payload.file_urls.unwrap_or_default(),
            &payload.chat_number,
        )
        .await?;
    Ok(Json(res))
}

pub async fn get_bounty_chat_numbers(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<GetBountyChatNumbersResponse>, ApiError> {
    let chat_numbers = state.service.bounty.get_bounty_chat_numbers(&id).await?;
    let mut chat_info = Vec::new();

    for chat_number in &chat_numbers {
        if let Ok(info) = state
            .service
            .bounty
            .get_chat_number_info(&id, chat_number)
            .await
        {
            chat_info.push(info);
        }
    }

    Ok(Json(GetBountyChatNumbersResponse {
        chat_numbers,
        chat_info,
    }))
}

pub async fn mark_chat_as_read(
    Extension(user): Extension<User>,
    Path((id, chat_number)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state
        .service
        .bounty
        .mark_chat_as_read(&id, &chat_number, user.id)
        .await?;
    Ok(Json(res))
}

pub async fn create_bidder_chat(
    Extension(user): Extension<User>,
    Path((id, bidder_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    // Verify that the current user is the bounty creator (funder)
    let bounty = state.service.bounty.get_bounty_info_by_id(&id).await?;
    if bounty.user.id != user.id {
        return Err(DbError::Str(
            "Only the bounty creator can initiate chats with bidders".to_string(),
        )
        .into());
    }

    let bidder_uuid = uuid_from_str(&bidder_id)?;
    let chat_number = state
        .service
        .bounty
        .get_or_create_chat_number(user.id, &bounty.nerd_id, &id, bidder_uuid)
        .await?;

    Ok(Json(chat_number))
}

pub async fn get_similar_bounties(
    Path(id): Path<String>,
    Query(opts): Query<GetSimilarBountiesOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BountyInfo>>, ApiError> {
    let similar_bounties = state
        .service
        .bounty
        .get_similar_bounties(&id, opts.limit)
        .await?;
    Ok(Json(similar_bounties))
}

pub async fn get_bounty_chat_list(
    Extension(user): Extension<User>,
    Query(opts): Query<OffsetAndLimitOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BountyChatListResponse>>, ApiError> {
    let chat_list = state
        .service
        .bounty
        .get_bounty_chat_list(user.id, opts.offset, opts.limit)
        .await?;
    Ok(Json(chat_list))
}
