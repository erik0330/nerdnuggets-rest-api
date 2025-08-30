use crate::pool::DatabasePool;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::models::{Prediction, PredictionStatus, User};
use uuid::Uuid;

#[derive(Clone)]
pub struct PredictionRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl PredictionRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn get_prediction_by_id(&self, id: Uuid) -> Option<Prediction> {
        sqlx::query_as::<_, Prediction>("SELECT * FROM prediction WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_prediction_by_nerd_id(&self, nerd_id: &str) -> Option<Prediction> {
        sqlx::query_as::<_, Prediction>("SELECT * FROM prediction WHERE nerd_id = $1")
            .bind(nerd_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn check_nerd_id(&self, nerd_id: &str) -> bool {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM prediction WHERE nerd_id = $1",
            nerd_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await
        .map(|row| row.count.unwrap_or(0))
        .unwrap_or(0);
        count == 0
    }

    // pub async fn create_prediction(
    //     &self,
    //     user_id: Uuid,
    //     nerd_id: &str,
    //     contract_id: i64,
    //     title: String,
    //     description: String,
    //     upload_file: Option<String>,
    //     cover_photo: Option<String>,
    //     category: Uuid,
    //     difficulty: PredictionDifficulty,
    //     tags: Option<Vec<String>>,
    //     reward_amount: i32,
    //     reward_currency: String,
    //     deadline: NaiveDate,
    //     requirements: Vec<String>,
    //     deliverables: Vec<String>,
    //     evaluation_criteria: Vec<String>,
    //     by_milestone: bool,
    // ) -> Result<Prediction, SqlxError> {
    //     let prediction = sqlx::query_as::<_, Prediction>(
    //         "INSERT INTO prediction (user_id, nerd_id, contract_id, status, title, description, upload_file, cover_photo, category, difficulty, tags, reward_amount, reward_currency, deadline, requirements, deliverables, evaluation_criteria, by_milestone)
    //         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18) RETURNING *",
    //     )
    //     .bind(user_id)
    //     .bind(nerd_id)
    //     .bind(contract_id)
    //     .bind(PredictionStatus::PendingApproval)
    //     .bind(title)
    //     .bind(description)
    //     .bind(upload_file)
    //     .bind(cover_photo)
    //     .bind(category)
    //     .bind(difficulty)
    //     .bind(tags)
    //     .bind(reward_amount)
    //     .bind(reward_currency)
    //     .bind(deadline)
    //     .bind(requirements)
    //     .bind(deliverables)
    //     .bind(evaluation_criteria)
    //     .bind(by_milestone)
    //     .fetch_one(self.db_conn.get_pool())
    //     .await?;
    //     Ok(prediction)
    // }

    // pub async fn delete_prediction(
    //     &self,
    //     id: Uuid,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("DELETE FROM prediction WHERE id = $1")
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    pub async fn get_predictions(
        &self,
        title: Option<String>,
        status: Option<PredictionStatus>,
        category_id: Option<Uuid>,
        user: Option<&User>,
        is_mine: Option<bool>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Prediction>, SqlxError> {
        let mut filters = Vec::new();
        let mut index = 3;
        let mut query = format!("SELECT p.* FROM prediction p");
        if title.as_ref().map_or(false, |s| !s.is_empty()) {
            filters.push(format!("p.title ILIKE ${index}"));
            index += 1;
        }
        if status.is_some() {
            filters.push(format!("p.status = ${index}"));
            index += 1;
        }
        if category_id.is_some() {
            filters.push(format!("${index} = ANY(p.category)"));
            index += 1;
        }
        if is_mine.unwrap_or_default() && user.is_some() {
            query = format!(
                "{} JOIN prediction_placement pp ON p.proposal_id = pp.proposal_id AND p.number = pp.milestone_index AND pp.user_address ILIKE ${index} ",
                &query
            );
        }
        if !filters.is_empty() {
            query = format!("{} WHERE {}", &query, &filters.join(" AND "));
        }
        query = format!(
            "{} ORDER BY p.started_at, p.ended_at LIMIT $1 OFFSET $2",
            &query
        );
        let mut query = sqlx::query_as::<_, Prediction>(&query)
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
        if is_mine.unwrap_or_default() {
            if let Some(u) = user {
                query = query.bind(u.wallet_address.as_deref().unwrap_or_default());
            }
        }
        let predictions = query.fetch_all(self.db_conn.get_pool()).await?;
        Ok(predictions)
    }

    pub async fn update_prediction_pool_amounts(
        &self,
        proposal_id: i64,
        milestone_index: i64,
        nerd_amount: i64,
        predicts_success: bool,
    ) -> Result<bool, SqlxError> {
        let row = if predicts_success {
            sqlx::query(
                "UPDATE prediction SET 
                    pool_amount = pool_amount + $1, 
                    yes_pool_amount = yes_pool_amount + $1, 
                    count_predictors = count_predictors + 1, 
                    updated_at = now() 
                WHERE proposal_id = $2 AND number = $3",
            )
            .bind(nerd_amount)
            .bind(proposal_id)
            .bind(milestone_index as i16)
        } else {
            sqlx::query(
                "UPDATE prediction SET 
                    pool_amount = pool_amount + $1, 
                    no_pool_amount = no_pool_amount + $1, 
                    count_predictors = count_predictors + 1, 
                    updated_at = now() 
                WHERE proposal_id = $2 AND number = $3",
            )
            .bind(nerd_amount)
            .bind(proposal_id)
            .bind(milestone_index as i16)
        }
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_prediction_result(
        &self,
        proposal_id: i64,
        milestone_index: i16,
        predict_result: bool,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query(
            "UPDATE prediction SET 
                status = $1,
                predict_result = $2,
                updated_at = now() 
            WHERE proposal_id = $3 AND number = $4",
        )
        .bind(PredictionStatus::Completed)
        .bind(predict_result)
        .bind(proposal_id)
        .bind(milestone_index)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }
}
