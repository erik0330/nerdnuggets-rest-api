use crate::{pool::DatabasePool, BountyRepository, UserRepository, UtilRepository};
use chrono::{Datelike, NaiveDate, Utc};
use std::sync::Arc;
use types::{
    dto::{BountyCreateRequest, BountyUpdateRequest, SubmitBidRequest},
    error::{ApiError, DbError, UserError},
    models::{
        BidInfo, Bounty, BountyCommentInfo, BountyDifficulty, BountyInfo, BountyReviewType,
        BountyStatus, User,
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

    pub async fn get_bounty_by_id(&self, id: &str) -> Result<BountyInfo, ApiError> {
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
        self.bounty_to_info(&bounty).await
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
        if payload.by_milestone {
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

    pub async fn get_bids(
        &self,
        id: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BidInfo>, ApiError> {
        let bids = self
            .bounty_repo
            .get_bids(uuid_from_str(id)?, offset, limit)
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
            self.bounty_repo
                .submit_bounty_comment(user_id, bounty.id, &bounty.nerd_id, comment)
                .await
                .unwrap_or_default()
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
            .review_bounty(bounty.id, status, admin_notes, approved_at, rejected_at)
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Review bounty failed".to_string()).into());
        }
        Ok(true)
    }
}
