use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};

use types::dto::GetPredictionsOption;
use types::error::ApiError;
use types::models::{PredictionInfo, User};

pub async fn get_prediction_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<PredictionInfo>, ApiError> {
    let prediction = state.service.prediction.get_prediction_by_id(&id).await?;
    Ok(Json(prediction))
}

// pub async fn create_prediction(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<PredictionCreateRequest>,
// ) -> Result<Json<PredictionInfo>, ApiError> {
//     let prediction = state
//         .service
//         .prediction
//         .create_prediction(user.id, payload)
//         .await?;
//     Ok(Json(prediction))
// }

// pub async fn delete_prediction(
//     Extension(user): Extension<User>,
//     Path(id): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<bool>, ApiError> {
//     let res = state
//         .service
//         .prediction
//         .delete_prediction(&id, user.id)
//         .await?;
//     Ok(Json(res))
// }

pub async fn get_predictions(
    Extension(user): Extension<Option<User>>,
    Query(opts): Query<GetPredictionsOption>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PredictionInfo>>, ApiError> {
    let res = state
        .service
        .prediction
        .get_predictions(
            opts.title,
            opts.status,
            opts.category_id,
            user.map(|u| u.id),
            opts.is_mine,
            opts.offset,
            opts.limit,
        )
        .await?;
    Ok(Json(res))
}
