use crate::{pool::DatabasePool, repository::HolderRepository};
use std::sync::Arc;
use types::{
    dto::{HolderDto, HoldersListResponse},
    error::{ApiError, DbError},
};

#[derive(Clone)]
pub struct HolderService {
    holder_repo: HolderRepository,
}

impl HolderService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            holder_repo: HolderRepository::new(db_conn),
        }
    }

    pub async fn get_holders(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<HoldersListResponse, ApiError> {
        let offset_val = offset.unwrap_or(0);
        let limit_val = limit.unwrap_or(100);

        // Get holders and total count
        let holders = self
            .holder_repo
            .get_holders(offset, limit)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        let total_count = self
            .holder_repo
            .get_holders_count()
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        // Convert holders to DTOs
        let holder_dtos: Vec<HolderDto> = holders.into_iter().map(HolderDto::from).collect();

        // Calculate has_more
        let has_more = (offset_val + limit_val) < total_count as i32;

        Ok(HoldersListResponse {
            holders: holder_dtos,
            total_count,
            has_more,
        })
    }
}
