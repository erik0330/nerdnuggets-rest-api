use crate::pool::DatabasePool;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::{models::{Bid, BidMilestone, BidStatus, Bounty, BountyComment, BountyDifficulty, BountyMilestone, BountyStatus}, UserRoleType};
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

    pub async fn get_bids(
        &self,
        id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Bid>, SqlxError> {
        let bids = sqlx::query_as::<_, Bid>("SELECT * FROM bid WHERE bounty_id = $1 LIMIT $2 OFFSET $3")
            .bind(id)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.db_conn.get_pool())
            .await?;
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

    pub async fn review_bounty(
        &self,
        id: Uuid,
        status: BountyStatus,
        admin_notes: Option<String>,
        approved_at: Option<DateTime<Utc>>,
        rejected_at: Option<DateTime<Utc>>
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE bounty SET status = $1, admin_notes = $2, updated_at = $3, approved_at = $4, rejected_at = $5 WHERE id = $6")
            .bind(status)
            .bind(admin_notes)
            .bind(Utc::now())
            .bind(approved_at)
            .bind(rejected_at)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }
}
