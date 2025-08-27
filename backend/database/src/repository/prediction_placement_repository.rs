use crate::pool::DatabasePool;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::models::PredictionPlacement;
use uuid::Uuid;

#[derive(Clone)]
pub struct PredictionPlacementRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl PredictionPlacementRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_prediction_placement(
        &self,
        user_address: &str,
        proposal_id: i64,
        milestone_index: i64,
        predicts_success: bool,
        nerd_amount: i64,
        block_number: i64,
    ) -> Result<PredictionPlacement, SqlxError> {
        let prediction_placement = sqlx::query_as::<_, PredictionPlacement>(
            "INSERT INTO prediction_placement (user_address, proposal_id, milestone_index, predicts_success, nerd_amount, block_number) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
        .bind(user_address)
        .bind(proposal_id)
        .bind(milestone_index)
        .bind(predicts_success)
        .bind(nerd_amount)
        .bind(block_number)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(prediction_placement)
    }

    pub async fn get_prediction_placements_by_project_milestone(
        &self,
        project_id: i64,
        milestone_index: i64,
    ) -> Result<Vec<PredictionPlacement>, SqlxError> {
        let prediction_placements = sqlx::query_as::<_, PredictionPlacement>(
            "SELECT * FROM prediction_placement WHERE project_id = $1 AND milestone_index = $2 ORDER BY created_at DESC"
        )
        .bind(project_id)
        .bind(milestone_index)
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(prediction_placements)
    }

    pub async fn get_prediction_placements_by_user(
        &self,
        user_address: &str,
    ) -> Result<Vec<PredictionPlacement>, SqlxError> {
        let prediction_placements = sqlx::query_as::<_, PredictionPlacement>(
            "SELECT * FROM prediction_placement WHERE user_address = $1 ORDER BY created_at DESC",
        )
        .bind(user_address)
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(prediction_placements)
    }

    pub async fn get_prediction_placement_by_id(&self, id: Uuid) -> Option<PredictionPlacement> {
        sqlx::query_as::<_, PredictionPlacement>("SELECT * FROM prediction_placement WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_prediction_placement_by_user_proposal_milestone(
        &self,
        user_address: &str,
        proposal_id: i64,
        milestone_index: i64,
    ) -> Option<PredictionPlacement> {
        sqlx::query_as::<_, PredictionPlacement>(
            "SELECT * FROM prediction_placement WHERE user_address = $1 AND proposal_id = $2 AND milestone_index = $3"
        )
        .bind(user_address)
        .bind(proposal_id)
        .bind(milestone_index)
        .fetch_optional(self.db_conn.get_pool())
        .await
        .unwrap_or(None)
    }
}
