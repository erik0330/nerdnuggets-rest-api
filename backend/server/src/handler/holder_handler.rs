use crate::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use types::dto::{HoldersListResponse, OffsetAndLimitOption};
use types::error::ApiError;

pub async fn get_holders(
    Query(opts): Query<OffsetAndLimitOption>,
    State(state): State<AppState>,
) -> Result<Json<HoldersListResponse>, ApiError> {
    let response = state
        .service
        .holder
        .get_holders(opts.offset, opts.limit)
        .await?;
    Ok(Json(response))
}
