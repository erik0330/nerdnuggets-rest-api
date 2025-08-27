use crate::{pool::DatabasePool, PredictionPlacementRepository};
use std::sync::Arc;
use types::models::PredictionPlacement;

#[derive(Clone)]
pub struct PredictionPlacementService {
    prediction_placement_repo: PredictionPlacementRepository,
}

impl PredictionPlacementService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            prediction_placement_repo: PredictionPlacementRepository::new(db_conn),
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
    ) -> Result<PredictionPlacement, sqlx::Error> {
        self.prediction_placement_repo
            .create_prediction_placement(
                user_address,
                proposal_id,
                milestone_index,
                predicts_success,
                nerd_amount,
                block_number,
            )
            .await
    }

    pub async fn get_prediction_placements_by_project_milestone(
        &self,
        project_id: i64,
        milestone_index: i64,
    ) -> Result<Vec<PredictionPlacement>, sqlx::Error> {
        self.prediction_placement_repo
            .get_prediction_placements_by_project_milestone(project_id, milestone_index)
            .await
    }

    pub async fn get_prediction_placements_by_user(
        &self,
        user_address: &str,
    ) -> Result<Vec<PredictionPlacement>, sqlx::Error> {
        self.prediction_placement_repo
            .get_prediction_placements_by_user(user_address)
            .await
    }

    pub async fn get_prediction_placement_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Option<PredictionPlacement> {
        self.prediction_placement_repo
            .get_prediction_placement_by_id(id)
            .await
    }
}
