use crate::pool::DatabasePool;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::{models::{Bid, BidMilestone, BidMilestoneStatus, BidMilestoneSubmission, BidMilestoneSubmissionStatus, BidStatus, Bounty, BountyChat, BountyComment, BountyDifficulty, BountyMilestone, BountyMilestoneSubmission, BountyStatus, BountySubmissionStatus, BountyWorkSubmission}, UserRoleType};
use uuid::Uuid;

#[derive(Clone)]
pub struct BountyRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl BountyRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn get_bounty_by_id(&self, id: Uuid) -> Option<Bounty> {
        sqlx::query_as::<_, Bounty>("SELECT * FROM bounty WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_bounty_by_nerd_id(&self, nerd_id: &str) -> Option<Bounty> {
        sqlx::query_as::<_, Bounty>("SELECT * FROM bounty WHERE nerd_id = $1")
            .bind(nerd_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn check_nerd_id(&self, nerd_id: &str) -> bool {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM bounty WHERE nerd_id = $1",
            nerd_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await
        .map(|row| row.count.unwrap_or(0))
        .unwrap_or(0);
        count == 0
    }

    pub async fn create_bounty(
        &self,
        user_id: Uuid,
        nerd_id: &str,
        contract_id: i64,
        title: String,
        description: String,
        upload_file: Option<String>,
        cover_photo: Option<String>,
        category: Uuid,
        difficulty: BountyDifficulty,
        tags: Option<Vec<String>>,
        reward_amount: i32,
        reward_currency: String,
        deadline: NaiveDate,
        requirements: Vec<String>,
        deliverables: Vec<String>,
        evaluation_criteria: Vec<String>,
        by_milestone: bool,
    ) -> Result<Bounty, SqlxError> {
        let bounty = sqlx::query_as::<_, Bounty>(
            "INSERT INTO bounty (user_id, nerd_id, contract_id, status, title, description, upload_file, cover_photo, category, difficulty, tags, reward_amount, reward_currency, deadline, requirements, deliverables, evaluation_criteria, by_milestone)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18) RETURNING *",
        )
        .bind(user_id)
        .bind(nerd_id)
        .bind(contract_id)
        .bind(BountyStatus::PendingApproval)
        .bind(title)
        .bind(description)
        .bind(upload_file)
        .bind(cover_photo)
        .bind(category)
        .bind(difficulty)
        .bind(tags)
        .bind(reward_amount)
        .bind(reward_currency)
        .bind(deadline)
        .bind(requirements)
        .bind(deliverables)
        .bind(evaluation_criteria)
        .bind(by_milestone)        
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(bounty)
    }

    pub async fn update_bounty(
        &self,
        id: Uuid,
        title: String,
        description: String,
        reward_amount: i32,
        reward_currency: String,
        difficulty: BountyDifficulty,
        deadline: NaiveDate
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET title = $1, description = $2, reward_amount = $3, reward_currency = $4, difficulty = $5, deadline = $6 WHERE id = $7")
            .bind(title)
            .bind(description)
            .bind(reward_amount)
            .bind(reward_currency)
            .bind(difficulty)
            .bind(deadline)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn delete_bounty(
        &self,
        id: Uuid,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("DELETE FROM bounty WHERE id = $1")
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn create_milestone(
        &self,
        bounty_id: Uuid,
        number: i16,
        title: String,
        description: String,
        reward_amount: i32,
        timeline: Option<String>,
        requirements: Vec<String>,
        deliverables: Vec<String>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO bounty_milestone (bounty_id, number, title, description, reward_amount, timeline, requirements, deliverables) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(bounty_id)
            .bind(number)
            .bind(title)
            .bind(description)
            .bind(reward_amount)
            .bind(timeline)
            .bind(requirements)
            .bind(deliverables)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_milestones(&self, bounty_id: Uuid) -> Vec<BountyMilestone> {
        sqlx::query_as::<_, BountyMilestone>(
            "SELECT * FROM bounty_milestone WHERE bounty_id = $1 ORDER BY number, created_at",
        )
        .bind(bounty_id)
        .fetch_all(self.db_conn.get_pool())
        .await
        .unwrap_or(Vec::new())
    }

    pub async fn delete_milestones(&self, bounty_id: Uuid) -> Result<bool, SqlxError> {
        let _ = sqlx::query("DELETE FROM milestone WHERE bounty_id = $1")
            .bind(bounty_id)
            .execute(self.db_conn.get_pool())
            .await?;
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
    ) -> Result<Vec<Bounty>, SqlxError> {
        let mut filters = Vec::new();
        let mut index = 3;
        let mut query = format!("SELECT b.* FROM bounty b");
        if title.as_ref().map_or(false, |s| !s.is_empty()) {
            filters.push(format!("b.title ILIKE ${index}"));
            index += 1;
        }
        if status.is_some() {
            filters.push(format!("b.status = ${index}"));
            index += 1;
        }
        if category_id.is_some() {
            filters.push(format!("b.category = ${index}"));
            index += 1;
        }
        if difficulty.is_some() {
            filters.push(format!("b.difficulty = ${index}"));
            index += 1;
        }
        if role.as_deref().unwrap_or_default() == UserRoleType::Funder.to_string() {
            if is_mine.unwrap_or_default() {
                if user_id.is_some() {
                    filters.push(format!("b.user_id = ${index}"));
                } else {
                    return Ok(Vec::new());
                }
            }
        }
        if !filters.is_empty() {
            query = format!("{} WHERE {}", &query, &filters.join(" AND "));
        }
        query = format!("{} ORDER BY b.updated_at DESC LIMIT $1 OFFSET $2", &query);
        let mut query = sqlx::query_as::<_, Bounty>(&query)
            .bind(limit.unwrap_or(5))
            .bind(offset.unwrap_or(0));
        if let Some(t) = title.as_ref().filter(|s| !s.is_empty()) {
            query = query.bind(format!("%{}%", t));
        }
        if let Some(s) = status {
            query = query.bind(s);
        }
        if let Some(c) = category_id {
            query = query.bind(c)
        }
        if let Some(d) = difficulty {
            query = query.bind(d)
        }
        if role.as_deref().unwrap_or_default() == UserRoleType::Funder.to_string() {
            if is_mine.unwrap_or_default() {
                if let Some(user_id) = user_id {
                    query = query.bind(user_id);
                }
            }
        }
        let bounties = query.fetch_all(self.db_conn.get_pool()).await?;
        Ok(bounties)
    }

    pub async fn get_bid_by_id(
        &self,
        bid_id: Uuid
    ) -> Result<Bid, SqlxError> {
        let bid = sqlx::query_as::<_, Bid>("SELECT * FROM bid WHERE id = $1")
            .bind(bid_id)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(bid)
    }

    pub async fn get_bids(
        &self,
        id: Uuid,
        status: Option<BidStatus>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Bid>, SqlxError> {
        let bids = if let Some(status) = status {
            sqlx::query_as::<_, Bid>("SELECT * FROM bid WHERE bounty_id = $1 AND status = $2 LIMIT $3 OFFSET $4")
                .bind(id)
                .bind(status)
                .bind(limit.unwrap_or(5))
                .bind(offset.unwrap_or(0))
                .fetch_all(self.db_conn.get_pool())
                .await?
        } else {
            sqlx::query_as::<_, Bid>("SELECT * FROM bid WHERE bounty_id = $1 LIMIT $2 OFFSET $3")
                .bind(id)
                .bind(limit.unwrap_or(5))
                .bind(offset.unwrap_or(0))
                .fetch_all(self.db_conn.get_pool())
                .await?
        };
        Ok(bids)
    }
    
    pub async fn get_my_bids(
        &self,
        user_id: Uuid,
        status: Option<BidStatus>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Bid>, SqlxError> {
        let bids = if let Some(status) = status {
            sqlx::query_as::<_, Bid>("SELECT * FROM bid WHERE user_id = $1 AND status = $2 LIMIT $3 OFFSET $4")
                .bind(user_id)
                .bind(status)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.db_conn.get_pool())
                .await?
        } else {
            sqlx::query_as::<_, Bid>("SELECT * FROM bid WHERE user_id = $1 LIMIT $2 OFFSET $3")
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.db_conn.get_pool())
                .await?
        };
        Ok(bids)
    }

    pub async fn create_bid(
        &self,
        bounty_id: Uuid,
        nerd_id: &str,
        user_id: Uuid,
        title: String,
        description: String,
        bid_amount: i32,
        timeline: String,
        technical_approach: String,
        relevant_experience: String,
        budget_breakdown: String,
        upload_files: Vec<String>,
    ) -> Result<Bid, SqlxError> {
        let bid = sqlx::query_as::<_, Bid>("INSERT INTO bid (bounty_id, nerd_id, user_id, status, title, description, bid_amount, timeline, technical_approach, relevant_experience, budget_breakdown, upload_files) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *")
            .bind(bounty_id)
            .bind(nerd_id)
            .bind(user_id)
            .bind(BidStatus::Submitted)
            .bind(title)
            .bind(description)
            .bind(bid_amount)
            .bind(timeline)
            .bind(technical_approach)
            .bind(relevant_experience)
            .bind(budget_breakdown)
            .bind(upload_files)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(bid)
    }

    pub async fn select_as_winner(
        &self,
        bounty_id: Uuid,
        bid_id: Uuid,    
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bid SET status = $1, updated_at = $2, accepted_at = $2 WHERE id = $3")
            .bind(BidStatus::InProgress)
            .bind(Utc::now())
            .bind(bid_id)
            .execute(self.db_conn.get_pool())
            .await?;
        let res = row.rows_affected() == 1;
        if res {
            let _ = sqlx::query("UPDATE bid SET status = $1, updated_at = $2, rejected_at = $2 WHERE bounty_id = $3 AND id != $4 AND (status = $5 OR status = $6)")
                .bind(BidStatus::Rejected)
                .bind(Utc::now())
                .bind(bounty_id)
                .bind(bid_id)
                .bind(BidStatus::Submitted)
                .bind(BidStatus::UnderReview)
                .execute(self.db_conn.get_pool())
                .await?;
            
            // Update the first milestone status to InProgress
            let _ = self.update_first_milestone_to_in_progress(bid_id).await;
        }
        Ok(res)
    }

    pub async fn reject_bid(
        &self,
        bid_id: Uuid
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bid SET status = $1, updated_at = $2, rejected_at = $2 WHERE id = $3")
            .bind(BidStatus::Rejected)
            .bind(Utc::now())
            .bind(bid_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_bid_milestones(
        &self,
        bid_id: Uuid
    ) -> Result<Vec<BidMilestone>, SqlxError> {
        let milestones = sqlx::query_as::<_, BidMilestone>("SELECT * FROM bid_milestone WHERE bid_id = $1 ORDER BY number")
            .bind(bid_id)
            .fetch_all(self.db_conn.get_pool())
            .await?;
        Ok(milestones)
    }

    pub async fn get_bid_milestone_by_id(
        &self,
        milestone_id: Uuid
    ) -> Option<BidMilestone> {
        sqlx::query_as::<_, BidMilestone>("SELECT * FROM bid_milestone WHERE id = $1")
            .bind(milestone_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_winning_bid_milestones_by_bounty_id(
        &self,
        bounty_id: Uuid
    ) -> Result<Vec<BidMilestone>, SqlxError> {
        let bid_milestones = sqlx::query_as::<_, BidMilestone>(
            "SELECT bm.* FROM bid_milestone bm 
             INNER JOIN bid b ON bm.bid_id = b.id 
             WHERE bm.bounty_id = $1 AND (b.status = $2 OR b.status = $3) 
             ORDER BY bm.number"
        )
        .bind(bounty_id)
        .bind(BidStatus::Accepted)
        .bind(BidStatus::InProgress)
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(bid_milestones)
    }

    pub async fn create_bid_milestone(
        &self,
        bid_id: Uuid,
        bounty_id: Uuid,
        nerd_id: &str,
        number: i16,
        title: String,
        description: String,
        amount: i32,
        timeline: String,
    ) -> Result<BidMilestone, SqlxError> {
        let bid_milestone = sqlx::query_as::<_, BidMilestone>("INSERT INTO bid_milestone (bid_id, bounty_id, nerd_id, number, title, description, amount, timeline) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(bid_id)
            .bind(bounty_id)
            .bind(nerd_id)
            .bind(number)
            .bind(title)
            .bind(description)
            .bind(amount)
            .bind(timeline)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(bid_milestone)
    }

    pub async fn update_first_milestone_to_in_progress(
        &self,
        bid_id: Uuid,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bid_milestone SET status = $1, updated_at = $2 WHERE bid_id = $3 AND number = 1")
            .bind(BidMilestoneStatus::InProgress)
            .bind(Utc::now())
            .bind(bid_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_bounty_comments(
        &self,
        id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BountyComment>, SqlxError> {
        let bounty_comments = sqlx::query_as::<_, BountyComment>(
            "SELECT * FROM bounty_comment WHERE bounty_id = $1 ORDER BY created_at LIMIT $2 OFFSET $3",
        )
        .bind(id)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(bounty_comments)
    }

    pub async fn submit_bounty_comment(
        &self,
        user_id: Uuid,
        bounty_id: Uuid,
        nerd_id: &str,
        comment: &str,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO bounty_comment (user_id, bounty_id, nerd_id, comment) VALUES ($1, $2, $3, $4)")
            .bind(user_id)
            .bind(bounty_id)
            .bind(nerd_id)
            .bind(comment)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_bounty_status(
        &self,
        id: Uuid,
        status: BountyStatus,
        admin_notes: Option<String>,
        approved_at: Option<DateTime<Utc>>,
        rejected_at: Option<DateTime<Utc>>,
        started_at: Option<DateTime<Utc>>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET status = $1, admin_notes = $2, updated_at = $3, approved_at = $4, rejected_at = $5, started_at = $6 WHERE id = $7")
            .bind(status)
            .bind(admin_notes)
            .bind(Utc::now())
            .bind(approved_at)
            .bind(rejected_at)
            .bind(started_at)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_bounty_status_with_reason(
        &self,
        id: Uuid,
        status: BountyStatus,
        admin_notes: Option<String>,
        approved_at: Option<DateTime<Utc>>,
        canceled_at: Option<DateTime<Utc>>,
        started_at: Option<DateTime<Utc>>,
        cancellation_reason: Option<String>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET status = $1, admin_notes = $2, updated_at = $3, approved_at = $4, canceled_at = $5, started_at = $6, cancellation_reason = $7 WHERE id = $8")
            .bind(status)
            .bind(admin_notes)
            .bind(Utc::now())
            .bind(approved_at)
            .bind(canceled_at)
            .bind(started_at)
            .bind(cancellation_reason)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_bounty_arweave_tx_id(
        &self,
        id: Uuid,
        arweave_tx_id: &str,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET arweave_tx_id = $1 WHERE id = $2")
            .bind(arweave_tx_id)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }
    
    pub async fn get_bounty_chats(
        &self,
        chat_number: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<BountyChat>, SqlxError> {
        let bounty_chats = sqlx::query_as::<_, BountyChat>(
            "SELECT * FROM bounty_chat WHERE chat_number = $1 ORDER BY created_at LIMIT $2 OFFSET $3",
        )
        .bind(chat_number)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(bounty_chats)
    }

    pub async fn send_bounty_chat(
        &self,
        sender_id: Uuid,
        receiver_id: Uuid,
        bounty_id: Uuid,
        nerd_id: &str,
        chat_number: &str,
        message: &str,
        file_urls: Vec<String>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO bounty_chat (sender_id, receiver_id, bounty_id, nerd_id, chat_number, message, file_urls) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(sender_id)
            .bind(receiver_id)
            .bind(bounty_id)
            .bind(nerd_id)
            .bind(chat_number)
            .bind(message)
            .bind(file_urls)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_bounty_chat_numbers(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<String>, SqlxError> {
        let chat_numbers = sqlx::query_scalar!(
            "SELECT DISTINCT chat_number FROM bounty_chat WHERE sender_id = $1 OR receiver_id = $1 ORDER BY chat_number",
            user_id
        ).fetch_all(self.db_conn.get_pool()).await?;
        Ok(chat_numbers)
    }

    pub async fn get_chat_number_info(
        &self,
        chat_number: &str,
    ) -> Result<Option<(String, Uuid, Option<String>, String, Uuid, Option<String>, String, Option<DateTime<Utc>>, i64, Uuid, String, Option<String>)>, SqlxError> {
        let result = sqlx::query!(
            r#"
            SELECT 
                sender_user.name as sender_name,
                sender_user.id as sender_id,
                sender_user.avatar_url as sender_avatar,
                receiver_user.name as receiver_name,
                receiver_user.id as receiver_id,
                receiver_user.avatar_url as receiver_avatar,
                bc.message as last_message,
                bc.created_at as last_message_time,
                (
                    SELECT COUNT(*)::int 
                    FROM bounty_chat 
                    WHERE chat_number = $1 AND is_read = false AND receiver_id = bc.receiver_id
                ) as unread_count,
                b.id as bounty_id,
                b.nerd_id as bounty_nerd_id,
                b.title as bounty_title
            FROM bounty_chat bc
            JOIN users sender_user ON bc.sender_id = sender_user.id
            JOIN users receiver_user ON bc.receiver_id = receiver_user.id
            JOIN bounty b ON bc.bounty_id = b.id
            WHERE bc.chat_number = $1
            ORDER BY bc.created_at DESC
            LIMIT 1
            "#,
            chat_number
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(result.map(|row| (
            row.sender_name.unwrap_or_default(),
            row.sender_id,
            row.sender_avatar,
            row.receiver_name.unwrap_or_default(),
            row.receiver_id,
            row.receiver_avatar,
            row.last_message,
            row.last_message_time,
            row.unread_count.unwrap_or(0) as i64,
            row.bounty_id,
            row.bounty_nerd_id,
            row.bounty_title,
        )))
    }

    pub async fn mark_chat_as_read(
        &self,
        chat_number: &str,
        user_id: Uuid,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query(
            "UPDATE bounty_chat SET is_read = true WHERE chat_number = $1 AND receiver_id = $2"
        )
        .bind(chat_number)
        .bind(user_id)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() > 0)
    }

    pub async fn get_similar_bounties(
        &self,
        bounty_id: Uuid,
        limit: Option<i32>,
    ) -> Result<Vec<Bounty>, SqlxError> {
        // First get the target bounty to extract its properties
        let target_bounty = self.get_bounty_by_id(bounty_id).await
            .ok_or_else(|| SqlxError::RowNotFound)?;

        let statuses = vec![BountyStatus::Open as i16, BountyStatus::PendingApproval as i16];
        // Find similar bounties based on category, difficulty, and overlapping tags
        let similar_bounties = sqlx::query_as::<_, Bounty>(
            "
            SELECT b.* FROM bounty b
            WHERE b.id != $1 
            AND b.status = ANY($6)
            AND (
                b.category = $2 
                OR b.difficulty = $3
                OR (
                    b.tags IS NOT NULL 
                    AND $4 IS NOT NULL 
                    AND EXISTS (
                        SELECT 1 FROM unnest(b.tags) tag1
                        WHERE EXISTS (
                            SELECT 1 FROM unnest($4) tag2
                            WHERE tag1 ILIKE '%' || tag2 || '%' OR tag2 ILIKE '%' || tag1 || '%'
                        )
                    )
                )
            )
            ORDER BY 
                CASE WHEN b.category = $2 THEN 3 ELSE 0 END +
                CASE WHEN b.difficulty = $3 THEN 2 ELSE 0 END +
                CASE WHEN b.tags IS NOT NULL AND $4 IS NOT NULL AND EXISTS (
                    SELECT 1 FROM unnest(b.tags) tag1
                    WHERE EXISTS (
                        SELECT 1 FROM unnest($4) tag2
                        WHERE tag1 ILIKE '%' || tag2 || '%' OR tag2 ILIKE '%' || tag1 || '%'
                    )
                ) THEN 1 ELSE 0 END DESC,
                b.created_at DESC
            LIMIT $5
            ",
        )
        .bind(bounty_id)
        .bind(target_bounty.category)
        .bind(target_bounty.difficulty)
        .bind(target_bounty.tags)
        .bind(limit.unwrap_or(5))
        .bind(statuses)
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(similar_bounties)
    }

    // pub async fn get_bounty_chat_list(
    //     &self,
    //     user_id: Uuid,
    //     offset: Option<i32>,
    //     limit: Option<i32>,
    // ) -> Result<Vec<(String, Uuid, String, String, BountyStatus, Uuid, String, Option<String>, DateTime<Utc>, String, DateTime<Utc>, i32)>, SqlxError> {
    //     let chat_list = sqlx::query!(
    //         r#"
    //         WITH chat_summary AS (
    //             SELECT 
    //                 bc.chat_number,
    //                 bc.bounty_id,
    //                 b.nerd_id,
    //                 b.title as bounty_title,
    //                 b.status as bounty_status,
    //                 b.user_id as funder_id,
    //                 u.name as funder_name,
    //                 u.avatar_url as funder_avatar,
    //                 MIN(bc.created_at) as first_message_at,
    //                 MAX(bc.created_at) as last_message_at,
    //                 (
    //                     SELECT message 
    //                     FROM bounty_chat 
    //                     WHERE bounty_id = bc.bounty_id 
    //                     AND chat_number = bc.chat_number 
    //                     ORDER BY created_at DESC 
    //                     LIMIT 1
    //                 ) as last_message,
    //                 (
    //                     SELECT COUNT(*)::int 
    //                     FROM bounty_chat 
    //                     WHERE bounty_id = bc.bounty_id 
    //                     AND chat_number = bc.chat_number 
    //                     AND is_read = false 
    //                     AND receiver_id = $1
    //                 ) as unread_count
    //             FROM bounty_chat bc
    //             JOIN bounty b ON bc.bounty_id = b.id
    //             JOIN users u ON b.user_id = u.id
    //             WHERE bc.sender_id = $1 OR bc.receiver_id = $1 OR b.user_id = $1
    //             GROUP BY bc.chat_number, bc.bounty_id, b.nerd_id, b.title, b.status, b.user_id, u.name, u.avatar_url
    //         )
    //         SELECT 
    //             chat_number,
    //             bounty_id,
    //             nerd_id,
    //             bounty_title,
    //             bounty_status,
    //             funder_id,
    //             funder_name,
    //             funder_avatar,
    //             first_message_at,
    //             last_message_at,
    //             last_message,
    //             unread_count
    //         FROM chat_summary
    //         ORDER BY last_message_at DESC
    //         LIMIT $2 OFFSET $3
    //         "#,
    //         user_id,
    //         limit.unwrap_or(20) as i64,
    //         offset.unwrap_or(0) as i64
    //     )
    //     .fetch_all(self.db_conn.get_pool())
    //     .await?;

    //     let result = chat_list
    //         .into_iter()
    //         .map(|row| (
    //             row.chat_number,
    //             row.bounty_id,
    //             row.nerd_id,
    //             row.bounty_title.unwrap_or_default(),
    //             match row.bounty_status.unwrap_or(0) {
    //                 0 => BountyStatus::PendingApproval,
    //                 1 => BountyStatus::Open,
    //                 2 => BountyStatus::Rejected,
    //                 3 => BountyStatus::InProgress,
    //                 4 => BountyStatus::UnderReview,
    //                 5 => BountyStatus::Completed,
    //                 6 => BountyStatus::Cancelled,
    //                 7 => BountyStatus::RequestRevision,
    //                 _ => BountyStatus::PendingApproval,
    //             },
    //             row.funder_id,
    //             row.funder_name.unwrap_or_default(),
    //             row.funder_avatar,
    //             row.first_message_at.unwrap_or_default(),
    //             row.last_message.unwrap_or_default(),
    //             row.last_message_at.unwrap_or_default(),
    //             row.unread_count.unwrap_or(0) as i32,
    //         ))
    //         .collect();

    //     Ok(result)
    // }

    // Bounty Work Submission Methods
    pub async fn create_bounty_work_submission(
        &self,
        bounty_id: Uuid,
        bid_id: Uuid,
        nerd_id: &str,
        user_id: Uuid,
        title: String,
        description: String,
        deliverable_files: Vec<String>,
        additional_notes: Option<String>,
    ) -> Result<BountyWorkSubmission, SqlxError> {
        let submission = sqlx::query_as::<_, BountyWorkSubmission>(
            "INSERT INTO bounty_work_submission (bounty_id, bid_id, nerd_id, user_id, title, description, deliverable_files, additional_notes, status) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
        )
        .bind(bounty_id)
        .bind(bid_id)
        .bind(nerd_id)
        .bind(user_id)
        .bind(title)
        .bind(description)
        .bind(deliverable_files)
        .bind(additional_notes)
        .bind(BountySubmissionStatus::Draft)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(submission)
    }

    pub async fn get_bounty_work_submission_by_id(
        &self,
        submission_id: Uuid,
    ) -> Option<BountyWorkSubmission> {
        sqlx::query_as::<_, BountyWorkSubmission>(
            "SELECT * FROM bounty_work_submission WHERE id = $1"
        )
        .bind(submission_id)
        .fetch_optional(self.db_conn.get_pool())
        .await
        .unwrap_or(None)
    }

    pub async fn get_bounty_work_submission_by_bid_id(
        &self,
        bid_id: Uuid,
    ) -> Option<BountyWorkSubmission> {
        sqlx::query_as::<_, BountyWorkSubmission>(
            "SELECT * FROM bounty_work_submission WHERE bid_id = $1"
        )
        .bind(bid_id)
        .fetch_optional(self.db_conn.get_pool())
        .await
        .unwrap_or(None)
    }

    pub async fn submit_bounty_work(
        &self,
        submission_id: Uuid,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query(
            "UPDATE bounty_work_submission SET status = $1, submitted_at = $2, updated_at = $2 WHERE id = $3"
        )
        .bind(BountySubmissionStatus::Submitted)
        .bind(Utc::now())
        .bind(submission_id)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_bounty_work_submission_status(
        &self,
        submission_id: Uuid,
        status: BountySubmissionStatus,
        admin_notes: Option<String>,
    ) -> Result<bool, SqlxError> {
        let now = Utc::now();
        let row = sqlx::query(
            "UPDATE bounty_work_submission SET status = $1, admin_notes = $2, updated_at = $3, 
             reviewed_at = CASE WHEN $1 IN (2, 3, 4, 5) THEN $3 ELSE reviewed_at END,
             approved_at = CASE WHEN $1 = 3 THEN $3 ELSE approved_at END,
             rejected_at = CASE WHEN $1 = 4 THEN $3 ELSE rejected_at END
             WHERE id = $4"
        )
        .bind(status)
        .bind(admin_notes)
        .bind(now)
        .bind(submission_id)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn create_bounty_milestone_submission(
        &self,
        work_submission_id: Uuid,
        milestone_number: i16,
        title: String,
        description: String,
        deliverable_files: Vec<String>,
        additional_notes: Option<String>,
    ) -> Result<BountyMilestoneSubmission, SqlxError> {
        let submission = sqlx::query_as::<_, BountyMilestoneSubmission>(
            "INSERT INTO bounty_milestone_submission (work_submission_id, milestone_number, title, description, deliverable_files, additional_notes, status) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
        .bind(work_submission_id)
        .bind(milestone_number)
        .bind(title)
        .bind(description)
        .bind(deliverable_files)
        .bind(additional_notes)
        .bind(BountySubmissionStatus::Draft)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(submission)
    }

    pub async fn get_bounty_milestone_submissions(
        &self,
        work_submission_id: Uuid,
    ) -> Result<Vec<BountyMilestoneSubmission>, SqlxError> {
        let submissions = sqlx::query_as::<_, BountyMilestoneSubmission>(
            "SELECT * FROM bounty_milestone_submission WHERE work_submission_id = $1 ORDER BY milestone_number"
        )
        .bind(work_submission_id)
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(submissions)
    }

    pub async fn update_bounty_milestone_submission_status(
        &self,
        milestone_submission_id: Uuid,
        status: BountySubmissionStatus,
    ) -> Result<bool, SqlxError> {
        let now = Utc::now();
        let row = sqlx::query(
            "UPDATE bounty_milestone_submission SET status = $1, updated_at = $2,
             submitted_at = CASE WHEN $1 = 1 THEN $2 ELSE submitted_at END,
             reviewed_at = CASE WHEN $1 IN (2, 3, 4, 5) THEN $2 ELSE reviewed_at END,
             approved_at = CASE WHEN $1 = 3 THEN $2 ELSE approved_at END,
             rejected_at = CASE WHEN $1 = 4 THEN $2 ELSE rejected_at END
             WHERE id = $3"
        )
        .bind(status)
        .bind(now)
        .bind(milestone_submission_id)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn increment_view_count(&self, id: Uuid) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET count_view = count_view + 1 WHERE id = $1")
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn increment_bid_count(&self, id: Uuid) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET count_bid = count_bid + 1 WHERE id = $1")
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn increment_comment_count(&self, id: Uuid) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET count_comment = count_comment + 1 WHERE id = $1")
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    // Bid Milestone Submission Methods
    pub async fn create_bid_milestone_submission(
        &self,
        bid_milestone_id: Uuid,
        bid_id: Uuid,
        bounty_id: Uuid,
        nerd_id: &str,
        milestone_number: i16,
        notes: String,
        attached_file_urls: Vec<String>,
    ) -> Result<BidMilestoneSubmission, SqlxError> {
        let submission = sqlx::query_as::<_, BidMilestoneSubmission>(
            "INSERT INTO bid_milestone_submission (bid_milestone_id, bid_id, bounty_id, nerd_id, milestone_number, notes, attached_file_urls, status) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
        )
        .bind(bid_milestone_id)
        .bind(bid_id)
        .bind(bounty_id)
        .bind(nerd_id)
        .bind(milestone_number)
        .bind(notes)
        .bind(attached_file_urls)
        .bind(BidMilestoneSubmissionStatus::Submitted)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(submission)
    }

    pub async fn get_bid_milestone_submission_by_id(
        &self,
        submission_id: Uuid,
    ) -> Option<BidMilestoneSubmission> {
        sqlx::query_as::<_, BidMilestoneSubmission>(
            "SELECT * FROM bid_milestone_submission WHERE id = $1"
        )
        .bind(submission_id)
        .fetch_optional(self.db_conn.get_pool())
        .await
        .unwrap_or(None)
    }

    pub async fn get_bid_milestone_submissions(
        &self,
        bid_milestone_id: Uuid,
    ) -> Result<Vec<BidMilestoneSubmission>, SqlxError> {
        let submissions = sqlx::query_as::<_, BidMilestoneSubmission>(
            "SELECT * FROM bid_milestone_submission WHERE bid_milestone_id = $1 ORDER BY milestone_number"
        )
        .bind(bid_milestone_id)
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(submissions)
    }

    pub async fn update_bid_milestone_submission_status(
        &self,
        submission_id: Uuid,
        status: BidMilestoneSubmissionStatus,
        feedback: Option<String>,
    ) -> Result<bool, SqlxError> {
        let now = Utc::now();
        let row = sqlx::query(
            "UPDATE bid_milestone_submission SET status = $1, updated_at = $2, reviewed_at = $2,
             approved_at = CASE WHEN $1 = 1 THEN $2 ELSE approved_at END,
             rejected_at = CASE WHEN $1 = 2 THEN $2 ELSE rejected_at END,
             feedback = $3
             WHERE id = $4"
        )
        .bind(status)
        .bind(now)
        .bind(feedback)
        .bind(submission_id)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_bid_milestone_status(
        &self,
        bid_milestone_id: Uuid,
        status: BidMilestoneStatus,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bid_milestone SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(status)
            .bind(Utc::now())
            .bind(bid_milestone_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_next_bid_milestone(
        &self,
        bid_id: Uuid,
        milestone_number: i16,
    ) -> Option<BidMilestone> {
        sqlx::query_as::<_, BidMilestone>(
            "SELECT * FROM bid_milestone WHERE bid_id = $1 AND number = $2 + 1 ORDER BY number ASC LIMIT 1"
        )
        .bind(bid_id)
        .bind(milestone_number)
        .fetch_optional(self.db_conn.get_pool())
        .await
        .unwrap_or(None)
    }

    pub async fn update_bid_status(
        &self,
        bid_id: Uuid,
        status: BidStatus,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bid SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(status)
            .bind(Utc::now())
            .bind(bid_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }
}
