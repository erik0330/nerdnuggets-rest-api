use crate::pool::DatabasePool;
use chrono::NaiveDate;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::models::{Bounty, BountyDifficulty, BountyStatus};
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
            "INSERT INTO bounty (user_id, nerd_id, contract_id, status, title, description, upload_file, category, difficulty, tags, reward_amount, reward_currency, deadline, requirements, deliverables, evaluation_criteria, by_milestone)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *",
        )
        .bind(user_id)
        .bind(nerd_id)
        .bind(contract_id)
        .bind(BountyStatus::PendingApproval)
        .bind(title)
        .bind(description)
        .bind(upload_file)
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

    // pub async fn get_milestones(&self, bounty_id: Uuid) -> Vec<Milestone> {
    //     sqlx::query_as::<_, Milestone>(
    //         "SELECT * FROM milestone WHERE bounty_id = $1 ORDER BY created_at",
    //     )
    //     .bind(bounty_id)
    //     .fetch_all(self.db_conn.get_pool())
    //     .await
    //     .unwrap_or(Vec::new())
    // }

    pub async fn create_milestone(
        &self,
        bounty_id: Uuid,
        number: i16,
        title: String,
        description: String,
        funding_amount: i32,
        days_after_start: i32,
        days_of_prediction: i32,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO milestone (bounty_id, number, title, description, funding_amount, days_after_start, days_of_prediction) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(bounty_id)
            .bind(number)
            .bind(title)
            .bind(description)
            .bind(funding_amount)
            .bind(days_after_start)
            .bind(days_of_prediction)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn delete_milestones(&self, bounty_id: Uuid) -> Result<bool, SqlxError> {
        let _ = sqlx::query("DELETE FROM milestone WHERE bounty_id = $1")
            .bind(bounty_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(true)
    }

    // pub async fn get_bountys(
    //     &self,
    //     title: Option<String>,
    //     status: Option<i16>,
    //     category_id: Option<Uuid>,
    //     role: Option<String>,
    //     user_id: Option<Uuid>,
    //     is_mine: Option<bool>,
    //     is_public: Option<bool>,
    //     offset: Option<i32>,
    //     limit: Option<i32>,
    // ) -> Result<Vec<BountyItem>, SqlxError> {
    //     let mut filters = Vec::new();
    //     let mut index = 3;
    //     let mut query = format!("SELECT p.id, p.nerd_id, p.user_id, p.title, p.description, p.cover_photo, p.category, p.status, p.funding_goal, p.duration, p.tags, p.funding_amount, p.count_contributors, p.created_at, p.updated_at, p.dao_at, p.started_at FROM bounty p");
    //     if title.as_ref().map_or(false, |s| !s.is_empty()) {
    //         filters.push(format!("p.title ILIKE ${index}"));
    //         index += 1;
    //     }
    //     if is_mine.unwrap_or_default() {
    //         if user_id.is_some() {
    //             filters.push(format!("p.user_id = ${index}"));
    //             index += 1;
    //             if status.is_some() {
    //                 filters.push(format!("p.status = ${index}"));
    //                 index += 1;
    //             }
    //         } else {
    //             return Ok(Vec::new());
    //         }
    //     } else {
    //         if is_public.unwrap_or_default() {
    //             if status.is_some() {
    //                 filters.push(format!("p.status = ${index}"));
    //                 index += 1;
    //             } else {
    //                 filters.push(format!("p.status = {}", BountyStatus::Funding.to_i16()));
    //                 filters.push(format!("p.status = {}", BountyStatus::Completed.to_i16()));
    //             }
    //         } else {
    //             match role.clone() {
    //                 Some(r) if r == UserRoleType::Admin.to_string() => {
    //                     if status.is_some() {
    //                         filters.push(format!("p.status = ${index}"));
    //                         index += 1;
    //                     } else {
    //                         filters
    //                             .push(format!("p.status != {}", BountyStatus::Creating.to_i16()));
    //                     }
    //                 }
    //                 Some(r) if r == UserRoleType::Editor.to_string() => {
    //                     if user_id.is_some() {
    //                         query = format!("{} JOIN bounty_editor pe ON p.id = pe.bounty_id AND pe.user_id = ${index} ", &query);
    //                         index += 1;
    //                         if let Some(s) = status {
    //                             match BountyStatus::from(s) {
    //                                 BountyStatus::UnderReview => {
    //                                     filters.push(format!(
    //                                         "p.status = {}",
    //                                         BountyStatus::UnderReview.to_i16()
    //                                     ));
    //                                 }
    //                                 _ => {
    //                                     filters.push(format!(
    //                                         "p.status > {}",
    //                                         BountyStatus::UnderReview.to_i16()
    //                                     ));
    //                                 }
    //                             }
    //                         }
    //                     } else {
    //                         return Ok(Vec::new());
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     if category_id.is_some() {
    //         filters.push(format!("${index} = ANY(p.category)"));
    //     }
    //     if !filters.is_empty() {
    //         query = format!("{} WHERE {}", &query, &filters.join(" AND "));
    //     }
    //     query = format!("{} ORDER BY p.updated_at DESC LIMIT $1 OFFSET $2", &query);
    //     let mut query = sqlx::query_as::<_, BountyItem>(&query)
    //         .bind(limit.unwrap_or(5))
    //         .bind(offset.unwrap_or(0));
    //     if let Some(title) = title.as_ref().filter(|s| !s.is_empty()) {
    //         query = query.bind(format!("%{}%", title));
    //     }
    //     if is_mine.unwrap_or_default() {
    //         if let Some(user_id) = user_id {
    //             query = query.bind(user_id);
    //             if let Some(s) = status {
    //                 query = query.bind(s);
    //             }
    //         }
    //     } else {
    //         if is_public.unwrap_or_default() {
    //             if let Some(s) = status {
    //                 query = query.bind(s);
    //             }
    //         } else {
    //             match role {
    //                 Some(r) if r == UserRoleType::Admin.to_string() => {
    //                     if let Some(s) = status {
    //                         query = query.bind(s);
    //                     }
    //                 }
    //                 Some(r) if r == UserRoleType::Editor.to_string() => {
    //                     if let Some(user_id) = user_id {
    //                         query = query.bind(user_id);
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     if let Some(category_id) = category_id {
    //         query = query.bind(category_id)
    //     }
    //     let bountys = query.fetch_all(self.db_conn.get_pool()).await?;
    //     Ok(bountys)
    // }

    // pub async fn update_bounty_status(
    //     &self,
    //     id: Uuid,
    //     status: &BountyStatus,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE bounty SET status = $1, updated_at = $2 WHERE id = $3")
    //         .bind(status.to_i16())
    //         .bind(Utc::now())
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn create_bounty_editor(
    //     &self,
    //     id: Uuid,
    //     nerd_id: &str,
    //     editor_id: Uuid,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query(
    //         "INSERT INTO bounty_editor (bounty_id, nerd_id, user_id) VALUES ($1, $2, $3)",
    //     )
    //     .bind(id)
    //     .bind(nerd_id)
    //     .bind(editor_id)
    //     .execute(self.db_conn.get_pool())
    //     .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_bounty_editor(
    //     &self,
    //     id: Uuid,
    //     editor_id: Uuid,
    //     status: &FeedbackStatus,
    //     feedback: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE bounty_editor SET status = $1, feedback = $2, updated_at = $3 WHERE bounty_id = $4 AND user_id = $5")
    //         .bind(status.to_i16())
    //         .bind(feedback)
    //         .bind(Utc::now())
    //         .bind(id)
    //         .bind(editor_id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn decide_admin(
    //     &self,
    //     id: Uuid,
    //     status: &BountyStatus,
    //     feedback: Option<String>,
    //     dao_at: Option<DateTime<Utc>>,
    //     started_at: Option<DateTime<Utc>>,
    // ) -> Result<Bounty, SqlxError> {
    //     let bounty = sqlx::query_as::<_, Bounty>(
    //         "UPDATE bounty SET status = $1, feedback = $2, updated_at = $3, dao_at = $4, started_at = $5 WHERE id = $6 RETURNING *",
    //     )
    //     .bind(status.to_i16())
    //     .bind(feedback)
    //     .bind(Utc::now())
    //     .bind(dao_at)
    //     .bind(started_at)
    //     .bind(id)
    //     .fetch_one(self.db_conn.get_pool())
    //     .await?;
    //     Ok(bounty)
    // }

    // pub async fn create_dao(&self, bounty: &Bounty) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("INSERT INTO dao (bounty_id, nerd_id, proposal_id, user_id, title, description, funding_goal) VALUES ($1, $2, $3, $4, $5, $6, $7)")
    //         .bind(bounty.id)
    //         .bind(&bounty.nerd_id)
    //         .bind(bounty.proposal_id)
    //         .bind(bounty.user_id)
    //         .bind(&bounty.title)
    //         .bind(&bounty.description)
    //         .bind(bounty.funding_goal)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_milestone_status(&self, id: Uuid, status: i16) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE milestone SET status = $1 WHERE id = $2")
    //         .bind(status)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_milestone(
    //     &self,
    //     id: Uuid,
    //     description: String,
    //     deliverables: Option<String>,
    //     challenges: Option<String>,
    //     next_steps: Option<String>,
    //     file_urls: Vec<String>,
    //     proof_status: i16,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE milestone SET description = $1, deliverables = $2, challenges = $3, next_steps = $4, file_urls = $5, proof_status = $6 WHERE id = $7")
    //         .bind(description)
    //         .bind(deliverables)
    //         .bind(challenges)
    //         .bind(next_steps)
    //         .bind(file_urls)
    //         .bind(proof_status)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn get_bounty_comments(
    //     &self,
    //     id: Uuid,
    //     offset: Option<i32>,
    //     limit: Option<i32>,
    // ) -> Result<Vec<BountyComment>, SqlxError> {
    //     let bounty_comments = sqlx::query_as::<_, BountyComment>(
    //         "SELECT * FROM bounty_comment WHERE bounty_id = $1 ORDER BY updated_at LIMIT $2 OFFSET $3",
    //     )
    //     .bind(id)
    //     .bind(limit.unwrap_or(10))
    //     .bind(offset.unwrap_or(0))
    //     .fetch_all(self.db_conn.get_pool())
    //     .await?;
    //     Ok(bounty_comments)
    // }

    // pub async fn submit_bounty_comment(
    //     &self,
    //     user_id: Uuid,
    //     bounty_id: Uuid,
    //     nerd_id: &str,
    //     comment: &str,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("INSERT INTO bounty_comment (user_id, bounty_id, nerd_id, comment) VALUES ($1, $2, $3, $4)")
    //         .bind(user_id)
    //         .bind(bounty_id)
    //         .bind(nerd_id)
    //         .bind(comment)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn get_daos(
    //     &self,
    //     title: Option<String>,
    //     status: Option<i16>,
    //     user_id: Option<Uuid>,
    //     is_mine: Option<bool>,
    //     offset: Option<i32>,
    //     limit: Option<i32>,
    // ) -> Result<Vec<Dao>, SqlxError> {
    //     let mut filters = Vec::new();
    //     let mut index = 3;
    //     let mut query = format!("SELECT d.* FROM dao d");
    //     if title.as_ref().map_or(false, |s| !s.is_empty()) {
    //         filters.push(format!("d.title ILIKE ${index}"));
    //         index += 1;
    //     }
    //     if is_mine.unwrap_or_default() {
    //         if user_id.is_some() {
    //             query = format!(
    //                 "{} LEFT JOIN dao_vote dv ON d.id = dv.dao_id AND dv.user_id = ${index} ",
    //                 &query
    //             );
    //         } else {
    //             return Ok(Vec::new());
    //         }
    //     } else if status.is_some() {
    //         filters.push(format!("d.status = ${index}"));
    //     }
    //     if !filters.is_empty() {
    //         query = format!("{} WHERE {}", &query, &filters.join(" AND "));
    //     }
    //     query = format!("{} ORDER BY d.updated_at DESC LIMIT $1 OFFSET $2", &query);
    //     let mut query = sqlx::query_as::<_, Dao>(&query)
    //         .bind(limit.unwrap_or(5))
    //         .bind(offset.unwrap_or(0));
    //     if let Some(title) = title.as_ref().filter(|s| !s.is_empty()) {
    //         query = query.bind(format!("%{}%", title));
    //     }
    //     if is_mine.unwrap_or_default() {
    //         if let Some(user_id) = user_id {
    //             query = query.bind(user_id);
    //         }
    //     } else if let Some(s) = status {
    //         query = query.bind(s);
    //     }
    //     let daos = query.fetch_all(self.db_conn.get_pool()).await?;
    //     Ok(daos)
    // }

    // pub async fn get_dao_by_id(&self, id: Uuid) -> Result<Dao, SqlxError> {
    //     let dao = sqlx::query_as::<_, Dao>("SELECT * FROM dao WHERE id = $1")
    //         .bind(id)
    //         .fetch_one(self.db_conn.get_pool())
    //         .await?;
    //     Ok(dao)
    // }

    // pub async fn get_my_dao_vote(&self, id: Uuid, user_id: Uuid) -> Option<DaoVote> {
    //     let dao_vote = sqlx::query_as::<_, DaoVote>(
    //         "SELECT * FROM dao_vote WHERE dao_id = $1 AND user_id = $2",
    //     )
    //     .bind(id)
    //     .bind(user_id)
    //     .fetch_optional(self.db_conn.get_pool())
    //     .await
    //     .unwrap_or_default();
    //     dao_vote
    // }

    // pub async fn submit_dao_vote(
    //     &self,
    //     id: Uuid,
    //     user_id: Uuid,
    //     status: i16,
    //     comment: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query(
    //         "UPDATE dao_vote SET status = $1, comment = $2 WHERE dao_id = $3 AND user_id = $4",
    //     )
    //     .bind(status)
    //     .bind(comment)
    //     .bind(id)
    //     .bind(user_id)
    //     .execute(self.db_conn.get_pool())
    //     .await?;
    //     Ok(row.rows_affected() == 1)
    // }
}
