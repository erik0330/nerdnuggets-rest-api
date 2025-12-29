use crate::{pool::DatabasePool, repository::HolderRepository};
use std::sync::Arc;
use types::{
    dto::{ClaimRequest, ClaimResponse, HolderDto, HoldersListResponse, TelegramLoginRequest, VerifyTelegramResponse},
    error::{ApiError, DbError},
    models::Holder,
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

    pub async fn get_holder_by_telegram_id(&self, telegram_id: &str) -> Option<Holder> {
        self.holder_repo.get_holder_by_telegram_id(telegram_id).await
    }

    pub async fn update_holder_claim(
        &self,
        telegram_id: &str,
        wallet_address: &str,
    ) -> Result<bool, ApiError> {
        self.holder_repo
            .update_holder_claim(telegram_id, wallet_address)
            .await
            .map_err(|err| DbError::Str(err.to_string()))
    }

    /// Set referrer for a holder
    pub async fn set_referrer(
        &self,
        telegram_id: &str,
        referrer_telegram_id: &str,
    ) -> Result<bool, ApiError> {
        self.holder_repo
            .set_referrer(telegram_id, referrer_telegram_id)
            .await
            .map_err(|err| DbError::Str(err.to_string()))
    }

    /// Add referral reward to referrer (adds to leaderboard points)
    pub async fn add_referral_reward(
        &self,
        referrer_telegram_id: &str,
        reward_amount: i64,
    ) -> Result<bool, ApiError> {
        self.holder_repo
            .add_referral_reward(referrer_telegram_id, reward_amount)
            .await
            .map_err(|err| DbError::Str(err.to_string()))
    }

    /// Get holder by wallet address
    pub async fn get_holder_by_wallet_address(&self, wallet_address: &str) -> Option<Holder> {
        self.holder_repo.get_holder_by_wallet_address(wallet_address).await
    }
}
