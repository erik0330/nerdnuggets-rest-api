use crate::{
    pool::DatabasePool, PredictionPlacementRepository, PredictionRepository, UserRepository,
    UtilRepository,
};
use std::sync::Arc;
use types::{
    error::{ApiError, DbError, UserError},
    models::{MyPrediction, Prediction, PredictionInfo, PredictionStatus, User},
};
use utils::commons::uuid_from_str;
use uuid::Uuid;

#[derive(Clone)]
pub struct PredictionService {
    prediction_repo: PredictionRepository,
    user_repo: UserRepository,
    util_repo: UtilRepository,
    prediction_placement_repo: PredictionPlacementRepository,
}

impl PredictionService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            prediction_repo: PredictionRepository::new(db_conn),
            user_repo: UserRepository::new(db_conn),
            util_repo: UtilRepository::new(db_conn),
            prediction_placement_repo: PredictionPlacementRepository::new(db_conn),
        }
    }

    pub async fn prediction_to_info(
        &self,
        prediction: &Prediction,
        current_user: Option<&User>,
    ) -> Result<PredictionInfo, ApiError> {
        let user = self
            .user_repo
            .get_user_by_id(prediction.user_id)
            .await
            .ok_or_else(|| ApiError::UserError(UserError::UserNotFound))?;
        let category = self
            .util_repo
            .get_category_by_ids(&prediction.category)
            .await;
        // Get myPredict if current user is provided and has wallet address
        let my_predict = if let Some(current_user) = current_user {
            if let Some(wallet_address) = &current_user.wallet_address {
                if let Some(placement) = self
                    .prediction_placement_repo
                    .get_prediction_placement_by_user_proposal_milestone(
                        wallet_address,
                        prediction.proposal_id,
                        prediction.number as i64,
                    )
                    .await
                {
                    Some(MyPrediction {
                        predicts_success: placement.predicts_success,
                        user_address: placement.user_address,
                        nerd_amount: placement.nerd_amount,
                        block_number: placement.block_number,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        Ok(prediction.to_info(user.to_info(), category, my_predict))
    }

    pub async fn get_prediction_by_id(
        &self,
        id: &str,
        user: Option<User>,
    ) -> Result<PredictionInfo, ApiError> {
        let prediction = if let Ok(id) = uuid_from_str(id) {
            self.prediction_repo
                .get_prediction_by_id(id)
                .await
                .ok_or_else(|| DbError::Str("Prediction not found".to_string()))?
        } else if id.starts_with("BT-") {
            self.prediction_repo
                .get_prediction_by_nerd_id(id)
                .await
                .ok_or_else(|| DbError::Str("Prediction not found".to_string()))?
        } else {
            return Err(DbError::Str("Invalid id format".to_string()).into());
        };
        self.prediction_to_info(&prediction, user.as_ref()).await
    }

    // pub async fn create_prediction(
    //     &self,
    //     user_id: Uuid,
    //     payload: PredictionCreateRequest,
    // ) -> Result<PredictionInfo, ApiError> {
    //     let (nerd_id, contract_id) = loop {
    //         let year = Utc::now().year();
    //         let rand = generate_random_number(1000, 9999);
    //         let nerd_id = format!("BT-{}-{}", year, rand);
    //         if self.prediction_repo.check_nerd_id(&nerd_id).await {
    //             break (nerd_id, year * 10000 + rand as i32);
    //         }
    //     };
    //     let deadline = NaiveDate::parse_from_str(&payload.deadline, "%m/%d/%Y")
    //         .map_err(|err| DbError::Str(err.to_string()))?;
    //     let prediction = self
    //         .prediction_repo
    //         .create_prediction(
    //             user_id,
    //             &nerd_id,
    //             contract_id as i64,
    //             payload.title,
    //             payload.description,
    //             payload.upload_file,
    //             payload.cover_photo,
    //             payload.category,
    //             payload.difficulty,
    //             payload.tags,
    //             payload.reward_amount,
    //             payload.reward_currency,
    //             deadline,
    //             payload.requirements,
    //             payload.deliverables,
    //             payload.evaluation_criteria,
    //             payload.by_milestone,
    //         )
    //         .await
    //         .map_err(|err| DbError::Str(err.to_string()))?;
    //     self.prediction_to_info(&prediction).await
    // }

    // pub async fn delete_prediction(&self, id: &str, user_id: Uuid) -> Result<bool, ApiError> {
    //     let id = uuid_from_str(id)?;
    //     let prediction = self
    //         .prediction_repo
    //         .get_prediction_by_id(id)
    //         .await
    //         .ok_or(DbError::Str("Prediction not found".to_string()))?;
    //     if prediction.user_id != user_id {
    //         return Err(DbError::Str("No permission".to_string()).into());
    //     }
    //     if !self
    //         .prediction_repo
    //         .delete_prediction(id)
    //         .await
    //         .unwrap_or_default()
    //     {
    //         return Err(DbError::Str("Delete prediction failed".to_string()).into());
    //     }
    //     Ok(true)
    // }

    pub async fn get_predictions(
        &self,
        user: Option<User>,
        title: Option<String>,
        status: Option<PredictionStatus>,
        category_id: Option<Uuid>,
        is_mine: Option<bool>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<PredictionInfo>, ApiError> {
        let predictions = self
            .prediction_repo
            .get_predictions(
                title,
                status,
                category_id,
                user.as_ref(),
                is_mine,
                offset,
                limit,
            )
            .await
            .map_err(|_| DbError::Str("Get predictions failed".to_string()))?;
        let mut prediction_infos = Vec::new();
        for prediction in predictions {
            if let Ok(prediction_info) = self.prediction_to_info(&prediction, user.as_ref()).await {
                prediction_infos.push(prediction_info);
            }
        }
        Ok(prediction_infos)
    }
}
