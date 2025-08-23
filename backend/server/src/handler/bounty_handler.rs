use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};

use third_party_api::arweave::upload_bounty_creation;
use types::dto::{
    BountyAction, BountyActionRequest, BountyCreateRequest, BountyUpdateRequest,
    CancelBountyRequest, GetBidsOption, GetBountyChatNumbersResponse, GetBountyChatsOption,
    GetBountysOption, GetMyBidsOption, GetMyBountyStatsResponse, GetSimilarBountiesOption,
    OffsetAndLimitOption, RejectBidMilestoneRequest, ReviewBidMilestoneSubmissionRequest,
    ReviewBountyRequest, ReviewBountyWorkSubmissionRequest, SendBountyChatRequest,
    SubmitBidMilestoneWorkRequest, SubmitBidRequest, SubmitBountyCommentRequest,
    SubmitBountyWorkRequest,
};
use types::error::{ApiError, DbError, ValidatedRequest};
use types::models::{
    BidInfo, BidMilestoneSubmission, BidMilestoneSubmissionStatus, BountyChatInfo,
    BountyCommentInfo, BountyInfo, BountyWorkSubmissionInfo, User,
};
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

    // Upload bounty metadata to Arweave
    let bounty_info = serde_json::json!({
        "id": bounty.id,
        "nerd_id": bounty.nerd_id,
        "contract_id": bounty.contract_id,
        "created_at": bounty.created_at,
    });

    let user_info = serde_json::json!({
        "id": user.id,
        "email": user.email,
    });

    match upload_bounty_creation(
        &bounty.id.to_string(),
        &bounty.nerd_id,
        &user.id.to_string(),
        &serde_json::to_value(&payload).unwrap(),
        &bounty_info,
        &user_info,
    )
    .await
    {
        Ok(arweave_id) => {
            // println!("Bounty uploaded to Arweave with ID: {}", arweave_id);
            // Store the Arweave ID in the database for future reference
            if let Err(_e) = state
                .service
                .bounty
                .update_bounty_arweave_tx_id(bounty.id, &arweave_id)
                .await
            {
                // println!("Failed to store Arweave transaction ID: {:?}", e);
            }
        }
        Err(_e) => {
            // println!("Failed to upload bounty to Arweave: {:?}", e);
        }
    }

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

pub async fn get_winning_bid_milestones(
    Path(bounty_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<types::models::BidMilestone>>, ApiError> {
    let milestones = state
        .service
        .bounty
        .get_winning_bid_milestones_by_bounty_id(&bounty_id)
        .await?;
    Ok(Json(milestones))
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
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state.service.bounty.select_as_winner(&id, user.id).await?;
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
    Query(opts): Query<GetBountyChatsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BountyChatInfo>>, ApiError> {
    let chats = state
        .service
        .bounty
        .get_bounty_chats(&opts.chat_number, opts.offset, opts.limit)
        .await?;
    Ok(Json(chats))
}

pub async fn send_bounty_chat(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SendBountyChatRequest>,
) -> Result<Json<bool>, ApiError> {
    // Get bounty information from chat number
    let chat_info = state
        .service
        .bounty
        .get_chat_number_info(user.id, &payload.chat_number)
        .await?;

    let res = state
        .service
        .bounty
        .send_bounty_chat(
            user.id,
            payload.receiver_id,
            &payload.message,
            payload.file_urls.unwrap_or_default(),
            &payload.chat_number,
            chat_info.bounty.id,
            &chat_info.bounty.nerd_id,
        )
        .await?;
    Ok(Json(res))
}

pub async fn get_bounty_chat_numbers(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<GetBountyChatNumbersResponse>, ApiError> {
    let chat_numbers = state
        .service
        .bounty
        .get_bounty_chat_numbers(user.id)
        .await?;
    let mut chat_info = Vec::new();
    for chat_number in &chat_numbers {
        if let Ok(info) = state
            .service
            .bounty
            .get_chat_number_info(user.id, chat_number)
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
    Path(chat_number): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let res = state
        .service
        .bounty
        .mark_chat_as_read(&chat_number, user.id)
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
        .get_or_create_chat_number(user.id, bidder_uuid, &id, &bounty.nerd_id)
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

// pub async fn get_bounty_chat_list(
//     Extension(user): Extension<User>,
//     Query(opts): Query<OffsetAndLimitOption>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<BountyChatListResponse>>, ApiError> {
//     let chat_list = state
//         .service
//         .bounty
//         .get_bounty_chat_list(user.id, opts.offset, opts.limit)
//         .await?;
//     Ok(Json(chat_list))
// }

// Bounty Work Submission Handlers
pub async fn save_bounty_work(
    Extension(user): Extension<User>,
    Path(bid_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SubmitBountyWorkRequest>,
) -> Result<Json<BountyWorkSubmissionInfo>, ApiError> {
    let submission = state
        .service
        .bounty
        .submit_bounty_work(&bid_id, user, payload)
        .await?;

    Ok(Json(submission))
}

pub async fn get_bounty_work_submission(
    Path(submission_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BountyWorkSubmissionInfo>, ApiError> {
    let submission = state
        .service
        .bounty
        .get_bounty_work_submission(&submission_id)
        .await?;
    Ok(Json(submission))
}

pub async fn finalize_bounty_work_submission(
    Extension(user): Extension<User>,
    Path(submission_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<bool>, ApiError> {
    let success = state
        .service
        .bounty
        .finalize_bounty_work_submission(&submission_id, user)
        .await?;
    Ok(Json(success))
}

pub async fn review_bounty_work_submission(
    Path(submission_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ReviewBountyWorkSubmissionRequest>,
) -> Result<Json<bool>, ApiError> {
    let status = match payload.status {
        types::models::BountyReviewType::Approve => types::models::BountySubmissionStatus::Approved,
        types::models::BountyReviewType::RequestRevision => {
            types::models::BountySubmissionStatus::RequestRevision
        }
        types::models::BountyReviewType::Reject => types::models::BountySubmissionStatus::Rejected,
    };

    let success = state
        .service
        .bounty
        .review_bounty_work_submission(&submission_id, status, payload.admin_notes)
        .await?;
    Ok(Json(success))
}

// Bid Milestone Submission Handlers
pub async fn submit_bid_milestone_work(
    Extension(user): Extension<User>,
    Path(bid_milestone_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<SubmitBidMilestoneWorkRequest>,
) -> Result<Json<BidMilestoneSubmission>, ApiError> {
    let submission = state
        .service
        .bounty
        .submit_bid_milestone_work(&bid_milestone_id, user.id, payload)
        .await?;

    Ok(Json(submission))
}

pub async fn get_bid_milestone_submissions(
    Path(bid_milestone_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BidMilestoneSubmission>>, ApiError> {
    let submissions = state
        .service
        .bounty
        .get_bid_milestone_submissions(&bid_milestone_id)
        .await?;
    Ok(Json(submissions))
}

pub async fn review_bid_milestone_submission(
    Path(submission_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<ReviewBidMilestoneSubmissionRequest>,
) -> Result<Json<bool>, ApiError> {
    let status = match payload.status {
        BidMilestoneSubmissionStatus::Approved => {
            types::models::BidMilestoneSubmissionStatus::Approved
        }
        BidMilestoneSubmissionStatus::Rejected => {
            types::models::BidMilestoneSubmissionStatus::Rejected
        }
        BidMilestoneSubmissionStatus::Submitted => {
            return Err(DbError::Str("Invalid review status".to_string()).into());
        }
    };

    let success = state
        .service
        .bounty
        .review_bid_milestone_submission(&submission_id, status, payload.feedback)
        .await?;
    Ok(Json(success))
}

pub async fn reject_bid_milestone(
    Extension(user): Extension<User>,
    Path(bid_milestone_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<RejectBidMilestoneRequest>,
) -> Result<Json<bool>, ApiError> {
    let success = state
        .service
        .bounty
        .reject_bid_milestone(user.id, &bid_milestone_id, payload.feedback)
        .await?;
    Ok(Json(success))
}

pub async fn handle_bounty_action(
    Extension(user): Extension<User>,
    Path(bounty_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<BountyActionRequest>,
) -> Result<Json<bool>, ApiError> {
    let success = state
        .service
        .bounty
        .handle_bounty_action(&bounty_id, user.id, payload.action, payload.admin_notes)
        .await?;
    Ok(Json(success))
}

pub async fn cancel_bounty(
    Extension(user): Extension<User>,
    Path(bounty_id): Path<String>,
    State(state): State<AppState>,
    ValidatedRequest(payload): ValidatedRequest<CancelBountyRequest>,
) -> Result<Json<bool>, ApiError> {
    let success = state
        .service
        .bounty
        .handle_bounty_action(&bounty_id, user.id, BountyAction::Cancel, payload.reason)
        .await?;
    Ok(Json(success))
}
