use crate::{pool::DatabasePool, BountyRepository, UserRepository, UtilRepository};
use chrono::{Datelike, NaiveDate, Utc};
use std::sync::Arc;
use types::{
    dto::{
        BountyChatBountyInfo, BountyChatUserInfo, BountyCreateRequest, BountyUpdateRequest,
        ChatNumberInfo, SubmitBidMilestoneWorkRequest, SubmitBidRequest, SubmitBountyWorkRequest,
    },
    error::{ApiError, DbError, UserError},
    models::{
        BidInfo, BidMilestoneStatus, BidMilestoneSubmission, BidStatus, Bounty, BountyChatInfo,
        BountyCommentInfo, BountyDifficulty, BountyInfo, BountyMilestoneSubmissionInfo,
        BountyReviewType, BountyStatus, BountyWorkSubmissionInfo, User,
    },
};
use utils::commons::{generate_random_number, uuid_from_str};
use uuid::Uuid;

#[derive(Clone)]
pub struct BountyService {
    bounty_repo: BountyRepository,
    user_repo: UserRepository,
    util_repo: UtilRepository,
}

impl BountyService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            bounty_repo: BountyRepository::new(db_conn),
            user_repo: UserRepository::new(db_conn),
            util_repo: UtilRepository::new(db_conn),
        }
    }

    pub async fn bounty_to_info(&self, bounty: &Bounty) -> Result<BountyInfo, ApiError> {
        let user = self
            .user_repo
            .get_user_by_id(bounty.user_id)
            .await
            .ok_or_else(|| ApiError::UserError(UserError::UserNotFound))?;
        let category = self.util_repo.get_category_by_id(bounty.category).await;
        let milestones = self.bounty_repo.get_milestones(bounty.id).await;
        Ok(bounty.to_info(user.to_info(), category, milestones))
    }

    pub async fn get_bounty_info_by_id(&self, id: &str) -> Result<BountyInfo, ApiError> {
        let bounty = self.get_bounty_by_id_or_nerd_id(id).await?;
        // Increment view count
        let _ = self.bounty_repo.increment_view_count(bounty.id).await;
        self.bounty_to_info(&bounty).await
    }

    pub async fn get_bounty_info_by_id_without_increment(
        &self,
        id: &str,
    ) -> Result<BountyInfo, ApiError> {
        let bounty = self.get_bounty_by_id_or_nerd_id(id).await?;
        self.bounty_to_info(&bounty).await
    }

    pub async fn get_bounty_by_id_or_nerd_id(&self, id: &str) -> Result<Bounty, ApiError> {
        let bounty = if let Ok(id) = uuid_from_str(id) {
            self.bounty_repo
                .get_bounty_by_id(id)
                .await
                .ok_or_else(|| DbError::Str("Bounty not found".to_string()))?
        } else if id.starts_with("BT-") {
            self.bounty_repo
                .get_bounty_by_nerd_id(id)
                .await
                .ok_or_else(|| DbError::Str("Bounty not found".to_string()))?
        } else {
            return Err(DbError::Str("Invalid id format".to_string()).into());
        };
        Ok(bounty)
    }

    pub async fn create_bounty(
        &self,
        user_id: Uuid,
        payload: BountyCreateRequest,
    ) -> Result<BountyInfo, ApiError> {
        let (nerd_id, contract_id) = loop {
            let year = Utc::now().year();
            let rand = generate_random_number(1000, 9999);
            let nerd_id = format!("BT-{}-{}", year, rand);
            if self.bounty_repo.check_nerd_id(&nerd_id).await {
                break (nerd_id, year * 10000 + rand as i32);
            }
        };
        let deadline = NaiveDate::parse_from_str(&payload.deadline, "%m/%d/%Y")
            .map_err(|err| DbError::Str(err.to_string()))?;
        let bounty = self
            .bounty_repo
            .create_bounty(
                user_id,
                &nerd_id,
                contract_id as i64,
                payload.title,
                payload.description,
                payload.upload_file,
                payload.cover_photo,
                payload.category,
                payload.difficulty,
                payload.tags,
                payload.reward_amount,
                payload.reward_currency,
                deadline,
                payload.requirements,
                payload.deliverables,
                payload.evaluation_criteria,
                payload.by_milestone,
            )
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        let mut number = 1;
        for milestone in payload.milestones.unwrap_or_default() {
            match self
                .bounty_repo
                .create_milestone(
                    bounty.id,
                    number,
                    milestone.title,
                    milestone.description,
                    milestone.reward_amount,
                    milestone.timeline,
                    milestone.requirements.unwrap_or_default(),
                    milestone.deliverables.unwrap_or_default(),
                )
                .await
            {
                Ok(f) if f => number += 1,
                Ok(f) => println!("{f}"),
                Err(e) => println!("{:?}", e),
            }
        }
        self.bounty_to_info(&bounty).await
    }

    pub async fn update_bounty(
        &self,
        id: &str,
        payload: BountyUpdateRequest,
    ) -> Result<bool, ApiError> {
        let id = uuid_from_str(id)?;
        let _ = self
            .bounty_repo
            .get_bounty_by_id(id)
            .await
            .ok_or(DbError::Str("Bounty not found".to_string()))?;

        // TODO: check the status of the bounty
        let deadline = NaiveDate::parse_from_str(&payload.deadline, "%m/%d/%Y")
            .map_err(|err| DbError::Str(err.to_string()))?;
        if !self
            .bounty_repo
            .update_bounty(
                id,
                payload.title,
                payload.description,
                payload.reward_amount,
                payload.reward_currency,
                payload.difficulty,
                deadline,
            )
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Update bounty failed".to_string()).into());
        }
        Ok(true)
    }

    pub async fn delete_bounty(&self, id: &str, user_id: Uuid) -> Result<bool, ApiError> {
        let id = uuid_from_str(id)?;
        let bounty = self
            .bounty_repo
            .get_bounty_by_id(id)
            .await
            .ok_or(DbError::Str("Bounty not found".to_string()))?;
        if bounty.user_id != user_id {
            return Err(DbError::Str("No permission".to_string()).into());
        }
        if !self.bounty_repo.delete_bounty(id).await.unwrap_or_default() {
            return Err(DbError::Str("Delete bounty failed".to_string()).into());
        }
        Ok(true)
    }

    pub async fn get_bounties(
        &self,
        title: Option<String>,
        status: Option<BountyStatus>,
        category_id: Option<Uuid>,
        difficulty: Option<BountyDifficulty>,
        role: Option<String>,
        user_id: Option<Uuid>,
        is_mine: Option<bool>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BountyInfo>, ApiError> {
        let bounties = self
            .bounty_repo
            .get_bounties(
                title,
                status,
                category_id,
                difficulty,
                role,
                user_id,
                is_mine,
                offset,
                limit,
            )
            .await
            .map_err(|_| DbError::Str("Get bounties failed".to_string()))?;
        let mut bounty_infos = Vec::new();
        for bounty in bounties {
            if let Ok(bounty_info) = self.bounty_to_info(&bounty).await {
                bounty_infos.push(bounty_info);
            }
        }
        Ok(bounty_infos)
    }

    pub async fn get_bids(
        &self,
        id: &str,
        status: Option<BidStatus>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BidInfo>, ApiError> {
        let bids = self
            .bounty_repo
            .get_bids(uuid_from_str(id)?, status, offset, limit)
            .await
            .map_err(|_| DbError::Str("Get bids failed".to_string()))?;
        let mut bid_infos = Vec::new();
        for bid in bids {
            if let Some(user) = self.user_repo.get_user_by_id(bid.user_id).await {
                let milestones = self
                    .bounty_repo
                    .get_bid_milestones(bid.id)
                    .await
                    .unwrap_or_default();
                bid_infos.push(bid.to_info(user.to_info(), milestones));
            }
        }
        Ok(bid_infos)
    }

    pub async fn get_winning_bid_milestones_by_bounty_id(
        &self,
        bounty_id: &str,
    ) -> Result<Vec<types::models::BidMilestone>, ApiError> {
        let bounty = self.get_bounty_by_id_or_nerd_id(bounty_id).await?;
        let milestones = self
            .bounty_repo
            .get_winning_bid_milestones_by_bounty_id(bounty.id)
            .await
            .map_err(|_| DbError::Str("Failed to get winning bid milestones".to_string()))?;
        Ok(milestones)
    }

    pub async fn get_my_bids(
        &self,
        user: User,
        status: Option<BidStatus>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BidInfo>, ApiError> {
        let bids = self
            .bounty_repo
            .get_my_bids(user.id, status, offset, limit)
            .await
            .map_err(|_| DbError::Str("Get my bids failed".to_string()))?;
        let mut bid_infos = Vec::new();
        for bid in bids {
            let milestones = self
                .bounty_repo
                .get_bid_milestones(bid.id)
                .await
                .unwrap_or_default();
            bid_infos.push(bid.to_info(user.to_info(), milestones));
        }
        Ok(bid_infos)
    }

    pub async fn submit_bid(
        &self,
        id: &str,
        user: User,
        payload: SubmitBidRequest,
    ) -> Result<BidInfo, ApiError> {
        let id = uuid_from_str(id)?;
        let bounty = self
            .bounty_repo
            .get_bounty_by_id(id)
            .await
            .ok_or(DbError::Str("Bounty not found".to_string()))?;
        if bounty.status != BountyStatus::Open {
            return Err(DbError::Str(format!(
                "Can't bid on this bounty because its status is not opened"
            ))
            .into());
        }
        let bid = self
            .bounty_repo
            .create_bid(
                id,
                &bounty.nerd_id,
                user.id,
                payload.title,
                payload.description,
                payload.bid_amount,
                payload.timeline,
                payload.technical_approach,
                payload.relevant_experience,
                payload.budget_breakdown,
                payload.upload_files,
            )
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        // Increment bid count
        let _ = self.bounty_repo.increment_bid_count(bounty.id).await;

        let mut number = 1;
        let mut milestones = Vec::new();
        for m in payload.milestones {
            if let Ok(res) = self
                .bounty_repo
                .create_bid_milestone(
                    bid.id,
                    bounty.id,
                    &bounty.nerd_id,
                    number,
                    m.title,
                    m.description,
                    m.amount,
                    m.timeline,
                )
                .await
            {
                milestones.push(res);
                number += 1;
            }
        }
        Ok(bid.to_info(user.to_info(), milestones))
    }

    pub async fn select_as_winner(&self, id: &str, user_id: Uuid) -> Result<bool, ApiError> {
        let bid_id = uuid_from_str(id)?;
        let bid = self
            .bounty_repo
            .get_bid_by_id(bid_id)
            .await
            .map_err(|_| DbError::Str("Bid not found".to_string()))?;
        let bounty = self
            .bounty_repo
            .get_bounty_by_id(bid.bounty_id)
            .await
            .ok_or(DbError::Str("Bounty not found".to_string()))?;
        if bounty.user_id != user_id {
            return Err(DbError::Str("You are not the owner of this bounty".to_string()).into());
        }
        if bounty.status != BountyStatus::Open {
            return Err(DbError::Str("The bounty is not open".to_string()).into());
        }
        if !self
            .bounty_repo
            .select_as_winner(bid.bounty_id, bid_id)
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Select the bid as winner failed".to_string()).into());
        }
        // Update the bounty status to InProgress
        let _ = self
            .bounty_repo
            .update_bounty_status(
                bounty.id,
                BountyStatus::InProgress,
                bounty.admin_notes,
                bounty.approved_at,
                bounty.rejected_at,
                Some(Utc::now()),
            )
            .await;
        Ok(true)
    }

    pub async fn reject_bid(&self, id: &str) -> Result<bool, ApiError> {
        if !self
            .bounty_repo
            .reject_bid(uuid_from_str(id)?)
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Reject the bid failed".to_string()).into());
        }
        Ok(true)
    }

    pub async fn get_bounty_comments(
        &self,
        id: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BountyCommentInfo>, ApiError> {
        let bounty_comments = self
            .bounty_repo
            .get_bounty_comments(uuid_from_str(id)?, offset, limit)
            .await
            .unwrap_or_default();
        let mut pc_infos = Vec::new();
        for pc in bounty_comments {
            if let Some(user) = self.user_repo.get_user_by_id(pc.user_id).await {
                pc_infos.push(pc.to_info(user.to_info()));
            }
        }
        Ok(pc_infos)
    }

    pub async fn submit_bounty_comment(
        &self,
        id: &str,
        user_id: Uuid,
        comment: &str,
    ) -> Result<bool, ApiError> {
        let res = if let Some(bounty) = self.bounty_repo.get_bounty_by_id(uuid_from_str(id)?).await
        {
            let comment_result = self
                .bounty_repo
                .submit_bounty_comment(user_id, bounty.id, &bounty.nerd_id, comment)
                .await
                .unwrap_or_default();

            if comment_result {
                // Increment comment count
                let _ = self.bounty_repo.increment_comment_count(bounty.id).await;
            }

            comment_result
        } else {
            false
        };
        Ok(res)
    }

    pub async fn review_bounty(
        &self,
        id: &str,
        status: BountyReviewType,
        admin_notes: Option<String>,
    ) -> Result<bool, ApiError> {
        let (status, approved_at, rejected_at) = match status {
            BountyReviewType::Approve => (BountyStatus::Open, Some(Utc::now()), None),
            BountyReviewType::RequestRevision => (BountyStatus::RequestRevision, None, None),
            BountyReviewType::Reject => (BountyStatus::Rejected, None, Some(Utc::now())),
        };
        let bounty = self
            .bounty_repo
            .get_bounty_by_id(uuid_from_str(id)?)
            .await
            .ok_or(DbError::Str("Bounty not found".to_string()))?;
        if bounty.status != BountyStatus::PendingApproval {
            return Err(DbError::Str(
                "The status of this bounty is not PendingApproval".to_string(),
            )
            .into());
        }
        if !self
            .bounty_repo
            .update_bounty_status(
                bounty.id,
                status,
                admin_notes,
                approved_at,
                rejected_at,
                None,
            )
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Review bounty failed".to_string()).into());
        }
        Ok(true)
    }

    pub async fn get_bounty_chats(
        &self,
        chat_number: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BountyChatInfo>, ApiError> {
        let bounty_chats = self
            .bounty_repo
            .get_bounty_chats(chat_number, offset, limit)
            .await
            .unwrap_or_default();
        let mut pc_infos = Vec::new();
        for pc in bounty_chats {
            if let Some(sender) = self.user_repo.get_user_by_id(pc.sender_id).await {
                if let Some(receiver) = self.user_repo.get_user_by_id(pc.receiver_id).await {
                    pc_infos.push(pc.to_info(sender.to_info(), receiver.to_info()));
                }
            }
        }
        Ok(pc_infos)
    }

    pub async fn send_bounty_chat(
        &self,
        sender_id: Uuid,
        receiver_id: Uuid,
        message: &str,
        file_urls: Vec<String>,
        chat_number: &str,
        bounty_id: Uuid,
        nerd_id: &str,
    ) -> Result<bool, ApiError> {
        let res = self
            .bounty_repo
            .send_bounty_chat(
                sender_id,
                receiver_id,
                bounty_id,
                nerd_id,
                chat_number,
                message,
                file_urls,
            )
            .await
            .unwrap_or_default();
        Ok(res)
    }

    pub async fn get_bounty_chat_numbers(&self, user_id: Uuid) -> Result<Vec<String>, ApiError> {
        let chat_numbers = self
            .bounty_repo
            .get_bounty_chat_numbers(user_id)
            .await
            .map_err(|_| DbError::Str("Failed to get chat numbers".to_string()))?;
        Ok(chat_numbers)
    }

    pub async fn get_chat_number_info(
        &self,
        user_id: Uuid,
        chat_number: &str,
    ) -> Result<ChatNumberInfo, ApiError> {
        if let Some((
            sender_name,
            sender_id,
            sender_avatar,
            receiver_name,
            receiver_id,
            receiver_avatar,
            last_message,
            last_message_time,
            unread_count,
            bounty_id,
            bounty_nerd_id,
            bounty_title,
        )) = self
            .bounty_repo
            .get_chat_number_info(chat_number)
            .await
            .map_err(|_| DbError::Str("Failed to get chat info".to_string()))?
        {
            let (name, avatar_url, other_id) = if sender_id == user_id {
                (receiver_name, receiver_avatar, receiver_id)
            } else {
                (sender_name, sender_avatar, sender_id)
            };

            // Get the user's username
            let user = self.user_repo.get_user_by_id(other_id).await;
            let username = user
                .map(|u| u.username.unwrap_or_default())
                .unwrap_or_default();

            Ok(ChatNumberInfo {
                chat_number: chat_number.to_string(),
                last_message,
                last_message_time,
                unread_count: unread_count as i32,
                bounty: BountyChatBountyInfo {
                    id: bounty_id,
                    nerd_id: bounty_nerd_id,
                    title: bounty_title,
                },
                user: BountyChatUserInfo {
                    id: other_id,
                    username,
                    name,
                    avatar: avatar_url,
                },
            })
        } else {
            Err(DbError::Str("Chat not found".to_string()).into())
        }
    }

    pub async fn mark_chat_as_read(
        &self,
        chat_number: &str,
        user_id: Uuid,
    ) -> Result<bool, ApiError> {
        let res = self
            .bounty_repo
            .mark_chat_as_read(chat_number, user_id)
            .await
            .unwrap_or_default();
        Ok(res)
    }

    pub fn generate_chat_number(
        &self,
        nerd_id: &str,
        sender_id: Uuid,
        receiver_id: Uuid,
    ) -> String {
        format!("{}-{}-{}", nerd_id, sender_id, receiver_id)
    }

    pub async fn get_or_create_chat_number(
        &self,
        sender_id: Uuid,
        receiver_id: Uuid,
        bounty_id: &str,
        nerd_id: &str,
    ) -> Result<String, ApiError> {
        let chat_number = self.generate_chat_number(nerd_id, sender_id, receiver_id);

        // Check if this chat number already exists
        let existing_chats = self
            .bounty_repo
            .get_bounty_chats(&chat_number, None, Some(1))
            .await
            .unwrap_or_default();

        if existing_chats.is_empty() {
            // Create an initial message to establish the chat
            let _ = self
                .bounty_repo
                .send_bounty_chat(
                    sender_id,
                    receiver_id,
                    uuid_from_str(bounty_id)?,
                    nerd_id,
                    &chat_number,
                    "Connected",
                    Vec::new(),
                )
                .await;
        }

        Ok(chat_number)
    }

    pub async fn get_similar_bounties(
        &self,
        id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<BountyInfo>, ApiError> {
        let bounty = self.get_bounty_by_id_or_nerd_id(id).await?;
        let similar_bounties = self
            .bounty_repo
            .get_similar_bounties(bounty.id, limit)
            .await
            .map_err(|_| DbError::Str("Failed to get similar bounties".to_string()))?;

        let mut bounty_infos = Vec::new();
        for bounty in similar_bounties {
            let bounty_info = self.bounty_to_info(&bounty).await?;
            bounty_infos.push(bounty_info);
        }

        Ok(bounty_infos)
    }

    // pub async fn get_bounty_chat_list(
    //     &self,
    //     user_id: Uuid,
    //     offset: Option<i32>,
    //     limit: Option<i32>,
    // ) -> Result<Vec<BountyChatListResponse>, ApiError> {
    //     let chat_list_data = self
    //         .bounty_repo
    //         .get_bounty_chat_list(user_id, offset, limit)
    //         .await
    //         .map_err(|_| DbError::Str("Failed to get bounty chat list".to_string()))?;

    //     let mut chat_list = Vec::new();
    //     for (
    //         chat_number,
    //         bounty_id,
    //         nerd_id,
    //         bounty_title,
    //         bounty_status,
    //         funder_id,
    //         funder_name,
    //         funder_avatar,
    //         created_at,
    //         last_message,
    //         last_message_at,
    //         unread_count,
    //     ) in chat_list_data
    //     {
    //         let funder = BountyChatUserInfo {
    //             id: funder_id,
    //             name: funder_name,
    //             avatar: funder_avatar,
    //         };

    //         let bounty = BountyChatBountyInfo {
    //             id: bounty_id,
    //             nerd_id,
    //             title: bounty_title,
    //             status: bounty_status,
    //         };

    //         chat_list.push(BountyChatListResponse {
    //             chat_number,
    //             bounty,
    //             funder,
    //             created_at,
    //             last_message,
    //             last_message_at,
    //             unread_count: unread_count as i32, // Convert from i64 to i32 for DTO
    //         });
    //     }

    //     Ok(chat_list)
    // }

    pub async fn update_bounty_arweave_tx_id(
        &self,
        bounty_id: Uuid,
        arweave_tx_id: &str,
    ) -> Result<bool, ApiError> {
        self.bounty_repo
            .update_bounty_arweave_tx_id(bounty_id, arweave_tx_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()).into())
    }

    // Bounty Work Submission Methods
    pub async fn submit_bounty_work(
        &self,
        bid_id: &str,
        user: User,
        payload: SubmitBountyWorkRequest,
    ) -> Result<BountyWorkSubmissionInfo, ApiError> {
        let bid_uuid = uuid_from_str(bid_id)?;

        // Get the bid to verify it exists and user is the winner
        let bid = self
            .bounty_repo
            .get_bid_by_id(bid_uuid)
            .await
            .map_err(|_| DbError::Str("Bid not found".to_string()))?;

        // Verify the user is the bid owner
        if bid.user_id != user.id {
            return Err(
                DbError::Str("You can only submit work for your own bid".to_string()).into(),
            );
        }

        // Verify the bid is accepted (winner) or in progress
        if bid.status != BidStatus::Accepted && bid.status != BidStatus::InProgress {
            return Err(
                DbError::Str("You can only submit work for accepted bids".to_string()).into(),
            );
        }

        // Get the bounty to get nerd_id
        let bounty = self
            .bounty_repo
            .get_bounty_by_id(bid.bounty_id)
            .await
            .ok_or_else(|| DbError::Str("Bounty not found".to_string()))?;

        // Check if work submission already exists
        if let Some(_existing) = self
            .bounty_repo
            .get_bounty_work_submission_by_bid_id(bid_uuid)
            .await
        {
            return Err(
                DbError::Str("Work submission already exists for this bid".to_string()).into(),
            );
        }

        // Create the work submission
        let work_submission = self
            .bounty_repo
            .create_bounty_work_submission(
                bounty.id,
                bid_uuid,
                &bounty.nerd_id,
                user.id,
                payload.title,
                payload.description,
                payload.deliverable_files,
                payload.additional_notes,
            )
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        // Create milestone submissions if provided
        let mut milestone_submissions = Vec::new();
        if let Some(milestone_payloads) = payload.milestone_submissions {
            for milestone_payload in milestone_payloads {
                let milestone_submission = self
                    .bounty_repo
                    .create_bounty_milestone_submission(
                        work_submission.id,
                        milestone_payload.milestone_number,
                        milestone_payload.title,
                        milestone_payload.description,
                        milestone_payload.deliverable_files,
                        milestone_payload.additional_notes,
                    )
                    .await
                    .map_err(|e| DbError::Str(e.to_string()))?;

                milestone_submissions.push(milestone_submission.to_info());
            }
        }

        // Convert to info format
        Ok(work_submission.to_info(user.to_info(), milestone_submissions))
    }

    pub async fn get_bounty_work_submission(
        &self,
        submission_id: &str,
    ) -> Result<BountyWorkSubmissionInfo, ApiError> {
        let submission_uuid = uuid_from_str(submission_id)?;

        let work_submission = self
            .bounty_repo
            .get_bounty_work_submission_by_id(submission_uuid)
            .await
            .ok_or_else(|| DbError::Str("Work submission not found".to_string()))?;

        let user = self
            .user_repo
            .get_user_by_id(work_submission.user_id)
            .await
            .ok_or_else(|| ApiError::UserError(UserError::UserNotFound))?;

        let milestone_submissions = self
            .bounty_repo
            .get_bounty_milestone_submissions(work_submission.id)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        let milestone_infos: Vec<BountyMilestoneSubmissionInfo> = milestone_submissions
            .into_iter()
            .map(|ms| ms.to_info())
            .collect();

        Ok(work_submission.to_info(user.to_info(), milestone_infos))
    }

    pub async fn finalize_bounty_work_submission(
        &self,
        submission_id: &str,
        user: User,
    ) -> Result<bool, ApiError> {
        let submission_uuid = uuid_from_str(submission_id)?;

        let work_submission = self
            .bounty_repo
            .get_bounty_work_submission_by_id(submission_uuid)
            .await
            .ok_or_else(|| DbError::Str("Work submission not found".to_string()))?;

        // Verify the user is the submission owner
        if work_submission.user_id != user.id {
            return Err(
                DbError::Str("You can only finalize your own work submission".to_string()).into(),
            );
        }

        // Submit the work (change status to Submitted)
        let success = self
            .bounty_repo
            .submit_bounty_work(submission_uuid)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        if success {
            // Update bounty status to UnderReview
            let bounty = self
                .bounty_repo
                .get_bounty_by_id(work_submission.bounty_id)
                .await
                .ok_or_else(|| DbError::Str("Bounty not found".to_string()))?;

            let _ = self
                .bounty_repo
                .update_bounty_status(
                    bounty.id,
                    BountyStatus::UnderReview,
                    bounty.admin_notes,
                    bounty.approved_at,
                    bounty.rejected_at,
                    Some(Utc::now()),
                )
                .await;
        }

        Ok(success)
    }

    pub async fn review_bounty_work_submission(
        &self,
        submission_id: &str,
        status: types::models::BountySubmissionStatus,
        admin_notes: Option<String>,
    ) -> Result<bool, ApiError> {
        let submission_uuid = uuid_from_str(submission_id)?;

        let work_submission = self
            .bounty_repo
            .get_bounty_work_submission_by_id(submission_uuid)
            .await
            .ok_or_else(|| DbError::Str("Work submission not found".to_string()))?;

        // Update the submission status
        let success = self
            .bounty_repo
            .update_bounty_work_submission_status(submission_uuid, status, admin_notes)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        if success {
            // Update bounty status based on submission status
            let bounty = self
                .bounty_repo
                .get_bounty_by_id(work_submission.bounty_id)
                .await
                .ok_or_else(|| DbError::Str("Bounty not found".to_string()))?;

            let new_bounty_status = match status {
                types::models::BountySubmissionStatus::Approved => BountyStatus::Completed,
                types::models::BountySubmissionStatus::Rejected => BountyStatus::InProgress,
                types::models::BountySubmissionStatus::RequestRevision => {
                    BountyStatus::RequestRevision
                }
                _ => bounty.status, // Keep current status for other cases
            };

            if new_bounty_status != bounty.status {
                let _ = self
                    .bounty_repo
                    .update_bounty_status(
                        bounty.id,
                        new_bounty_status,
                        bounty.admin_notes,
                        bounty.approved_at,
                        bounty.rejected_at,
                        Some(Utc::now()),
                    )
                    .await;
            }
        }

        Ok(success)
    }

    pub async fn submit_bid_milestone_work(
        &self,
        bid_milestone_id: &str,
        user_id: Uuid,
        payload: SubmitBidMilestoneWorkRequest,
    ) -> Result<BidMilestoneSubmission, ApiError> {
        let milestone_uuid = uuid_from_str(bid_milestone_id)?;

        // Get the bid milestone by ID
        let bid_milestone = self
            .bounty_repo
            .get_bid_milestone_by_id(milestone_uuid)
            .await
            .ok_or_else(|| DbError::Str("Bid milestone not found".to_string()))?;

        // Get the bid to verify the user is the bidder
        let bid = self
            .bounty_repo
            .get_bid_by_id(bid_milestone.bid_id)
            .await
            .map_err(|_| DbError::Str("Bid not found".to_string()))?;

        // Verify the user is the bid owner
        if bid.user_id != user_id {
            return Err(
                DbError::Str("You can only submit work for your own bid".to_string()).into(),
            );
        }

        // Verify the milestone status is InProgress
        if bid_milestone.status != BidMilestoneStatus::InProgress {
            return Err(DbError::Str(
                "You can only submit work for milestones that are in progress".to_string(),
            )
            .into());
        }

        // Create the milestone submission
        let submission = self
            .bounty_repo
            .create_bid_milestone_submission(
                milestone_uuid,
                bid.id,
                bid.bounty_id,
                &bid.nerd_id,
                bid_milestone.number,
                payload.notes,
                payload.attached_file_urls,
            )
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        Ok(submission)
    }

    pub async fn get_bid_milestone_submissions(
        &self,
        bid_milestone_id: &str,
    ) -> Result<Vec<BidMilestoneSubmission>, ApiError> {
        let bid_milestone_uuid = uuid_from_str(bid_milestone_id)?;

        let submission = self
            .bounty_repo
            .get_bid_milestone_submissions(bid_milestone_uuid)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        Ok(submission)
    }

    pub async fn review_bid_milestone_submission(
        &self,
        submission_id: &str,
        status: types::models::BidMilestoneSubmissionStatus,
        feedback: Option<String>,
    ) -> Result<bool, ApiError> {
        let submission_uuid = uuid_from_str(submission_id)?;

        let submission = self
            .bounty_repo
            .get_bid_milestone_submission_by_id(submission_uuid)
            .await
            .ok_or_else(|| DbError::Str("Milestone submission not found".to_string()))?;

        // Update the submission status
        let success = self
            .bounty_repo
            .update_bid_milestone_submission_status(submission_uuid, status, feedback)
            .await
            .map_err(|e| DbError::Str(e.to_string()))?;

        if success {
            // Update the milestone status based on submission status
            let new_milestone_status = match status {
                types::models::BidMilestoneSubmissionStatus::Approved => {
                    BidMilestoneStatus::Completed
                }
                _ => BidMilestoneStatus::InProgress,
            };

            let _ = self
                .bounty_repo
                .update_bid_milestone_status(submission.bid_milestone_id, new_milestone_status)
                .await;
        }

        Ok(success)
    }
}
