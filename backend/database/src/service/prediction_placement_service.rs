use crate::{pool::DatabasePool, PredictionPlacementRepository, PredictionRepository};
use std::sync::Arc;
use types::models::PredictionPlacement;

#[derive(Clone)]
pub struct PredictionPlacementService {
    prediction_placement_repo: PredictionPlacementRepository,
    prediction_repo: PredictionRepository,
}

impl PredictionPlacementService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            prediction_placement_repo: PredictionPlacementRepository::new(db_conn),
            prediction_repo: PredictionRepository::new(db_conn),
        }
    }

    pub async fn create_prediction_placement(
        &self,
        user_address: &str,
        proposal_id: i64,
        milestone_index: i64,
        predicts_success: bool,
        nerd_amount: u128,
        block_number: i64,
    ) -> Result<PredictionPlacement, sqlx::Error> {
        let placement = self
            .prediction_placement_repo
            .get_prediction_placement_by_user_proposal_milestone(
                user_address,
                proposal_id,
                milestone_index,
            )
            .await;
        if placement.is_some() {
            return Err(sqlx::Error::RowNotFound);
        }
        let nerd_amount = (nerd_amount as f64) / 10f64.powi(18);
        let nerd_amount_i64 = nerd_amount as i64;
        let milestone_index = milestone_index + 1;

        // Create the prediction placement
        let prediction_placement = self
            .prediction_placement_repo
            .create_prediction_placement(
                user_address,
                proposal_id,
                milestone_index,
                predicts_success,
                nerd_amount_i64,
                block_number,
            )
            .await?;

        // Update the prediction pool amounts and count
        if let Err(e) = self
            .prediction_repo
            .update_prediction_pool_amounts(
                proposal_id,
                milestone_index,
                nerd_amount_i64,
                predicts_success,
            )
            .await
        {
            // Log the error but don't fail the entire operation
            eprintln!("Failed to update prediction pool amounts: {}", e);
        }

        Ok(prediction_placement)
    }
}
