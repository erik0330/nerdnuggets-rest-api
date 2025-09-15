use crate::{pool::DatabasePool, repository::RewardRepository, UserRepository};
use std::sync::Arc;
use types::{
    error::{ApiError, DbError, UserError},
    models::{Reward, RewardInfo, RewardStats, RewardType},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct RewardService {
    reward_repo: RewardRepository,
    user_repo: UserRepository,
}

impl RewardService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            reward_repo: RewardRepository::new(db_conn),
            user_repo: UserRepository::new(db_conn),
        }
    }

    pub async fn create_reward(
        &self,
        user_id: Option<Uuid>,
        wallet_address: &str,
        reward_type: RewardType,
        points: i32,
    ) -> Result<Reward, ApiError> {
        self.reward_repo
            .create_reward(user_id, wallet_address, &reward_type.to_string(), points)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn get_reward_by_id(&self, reward_id: Uuid) -> Result<Reward, ApiError> {
        self.reward_repo
            .get_reward_by_id(reward_id)
            .await
            .ok_or_else(|| DbError::Str("Reward not found".to_string()).into())
    }

    pub async fn get_user_rewards(
        &self,
        user_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<RewardInfo>, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        let rewards = self
            .reward_repo
            .get_rewards_by_user_id(user_id, offset, limit)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        Ok(rewards.iter().map(|r| r.to_info()).collect())
    }

    pub async fn get_user_unclaimed_rewards(
        &self,
        user_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<RewardInfo>, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        let rewards = self
            .reward_repo
            .get_unclaimed_rewards_by_user_id(user_id, offset, limit)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        Ok(rewards.iter().map(|r| r.to_info()).collect())
    }

    pub async fn get_user_claimed_rewards(
        &self,
        user_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<RewardInfo>, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        let rewards = self
            .reward_repo
            .get_claimed_rewards_by_user_id(user_id, offset, limit)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        Ok(rewards.iter().map(|r| r.to_info()).collect())
    }

    pub async fn claim_reward(&self, user_id: Uuid, reward_id: Uuid) -> Result<Reward, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        // Get the reward and verify it belongs to the user
        let reward = self.get_reward_by_id(reward_id).await?;
        if reward.user_id != Some(user_id) {
            return Err(DbError::Str("Reward does not belong to user".to_string()).into());
        }

        if reward.claimed_at.is_some() {
            return Err(DbError::Str("Reward already claimed".to_string()).into());
        }

        self.reward_repo
            .claim_reward(reward_id, 0)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn get_user_reward_stats(&self, user_id: Uuid) -> Result<RewardStats, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        self.reward_repo
            .get_user_reward_stats(user_id)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn get_rewards_by_type(
        &self,
        reward_type: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<RewardInfo>, ApiError> {
        let rewards = self
            .reward_repo
            .get_rewards_by_type(reward_type, offset, limit)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        Ok(rewards.iter().map(|r| r.to_info()).collect())
    }

    pub async fn get_rewards_by_wallet_address(
        &self,
        wallet_address: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<RewardInfo>, ApiError> {
        let rewards = self
            .reward_repo
            .get_rewards_by_wallet_address(wallet_address, offset, limit)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        Ok(rewards.iter().map(|r| r.to_info()).collect())
    }

    pub async fn delete_reward(&self, user_id: Uuid, reward_id: Uuid) -> Result<bool, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        // Get the reward and verify it belongs to the user
        let reward = self.get_reward_by_id(reward_id).await?;
        if reward.user_id != Some(user_id) {
            return Err(DbError::Str("Reward does not belong to user".to_string()).into());
        }

        self.reward_repo
            .delete_reward(reward_id)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn update_reward_points(
        &self,
        user_id: Uuid,
        reward_id: Uuid,
        points: i32,
        amounts: i32,
    ) -> Result<Reward, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        // Get the reward and verify it belongs to the user
        let reward = self.get_reward_by_id(reward_id).await?;
        if reward.user_id != Some(user_id) {
            return Err(DbError::Str("Reward does not belong to user".to_string()).into());
        }

        if reward.claimed_at.is_some() {
            return Err(DbError::Str("Cannot update claimed reward".to_string()).into());
        }

        self.reward_repo
            .update_reward_points(reward_id, points, amounts)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn get_all_rewards(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<RewardInfo>, ApiError> {
        let rewards = self
            .reward_repo
            .get_all_rewards(offset, limit)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;

        Ok(rewards.iter().map(|r| r.to_info()).collect())
    }

    pub async fn get_rewards_count_by_user(&self, user_id: Uuid) -> Result<i64, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        self.reward_repo
            .get_rewards_count_by_user(user_id)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn get_unclaimed_rewards_count_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<i64, ApiError> {
        // Verify user exists
        self.user_repo
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound)?;

        self.reward_repo
            .get_unclaimed_rewards_count_by_user(user_id)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    // Helper method to award points for specific actions
    pub async fn award_start_dao_reward(
        &self,
        wallet_address: &str,
        points: i32,
    ) -> Result<Reward, ApiError> {
        let user = self.user_repo.get_user_by_wallet(wallet_address).await;
        self.create_reward(
            user.map(|u| u.id),
            wallet_address,
            RewardType::StartDao,
            points,
        )
        .await
    }

    pub async fn award_funding_process_reward(
        &self,
        wallet_address: &str,
        points: i32,
    ) -> Result<Reward, ApiError> {
        let user = self.user_repo.get_user_by_wallet(wallet_address).await;
        self.create_reward(
            user.map(|u| u.id),
            wallet_address,
            RewardType::FundingProcess,
            points,
        )
        .await
    }

    pub async fn award_fund_reward(
        &self,
        wallet_address: &str,
        points: i32,
    ) -> Result<Reward, ApiError> {
        let user = self.user_repo.get_user_by_wallet(wallet_address).await;
        self.create_reward(user.map(|u| u.id), wallet_address, RewardType::Fund, points)
            .await
    }

    pub async fn award_predict_reward(
        &self,
        wallet_address: &str,
        points: i32,
    ) -> Result<Reward, ApiError> {
        let user = self.user_repo.get_user_by_wallet(wallet_address).await;
        self.create_reward(
            user.map(|u| u.id),
            wallet_address,
            RewardType::Predict,
            points,
        )
        .await
    }
}
