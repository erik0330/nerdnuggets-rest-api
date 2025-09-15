use crate::pool::DatabasePool;
use chrono::Utc;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::models::{Reward, RewardStats};
use uuid::Uuid;

#[derive(Clone)]
pub struct RewardRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl RewardRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_reward(
        &self,
        user_id: Option<Uuid>,
        wallet_address: &str,
        reward_type: &str,
        points: i32,
    ) -> Result<Reward, SqlxError> {
        let reward = sqlx::query_as::<_, Reward>(
            "INSERT INTO rewards (user_id, wallet_address, reward_type, points)
            VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(user_id)
        .bind(wallet_address)
        .bind(reward_type)
        .bind(points)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(reward)
    }

    pub async fn get_reward_by_id(&self, id: Uuid) -> Option<Reward> {
        sqlx::query_as::<_, Reward>("SELECT * FROM rewards WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_rewards_by_user_id(
        &self,
        user_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Reward>, SqlxError> {
        let rewards = sqlx::query_as::<_, Reward>(
            "SELECT * FROM rewards WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(rewards)
    }

    pub async fn get_rewards_by_wallet_address(
        &self,
        wallet_address: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Reward>, SqlxError> {
        let rewards = sqlx::query_as::<_, Reward>(
            "SELECT * FROM rewards WHERE wallet_address = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(wallet_address)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(rewards)
    }

    pub async fn get_unclaimed_rewards_by_user_id(
        &self,
        user_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Reward>, SqlxError> {
        let rewards = sqlx::query_as::<_, Reward>(
            "SELECT * FROM rewards WHERE user_id = $1 AND claimed_at IS NULL ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(rewards)
    }

    pub async fn get_claimed_rewards_by_user_id(
        &self,
        user_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Reward>, SqlxError> {
        let rewards = sqlx::query_as::<_, Reward>(
            "SELECT * FROM rewards WHERE user_id = $1 AND claimed_at IS NOT NULL ORDER BY claimed_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(rewards)
    }

    pub async fn get_rewards_by_type(
        &self,
        reward_type: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Reward>, SqlxError> {
        let rewards = sqlx::query_as::<_, Reward>(
            "SELECT * FROM rewards WHERE reward_type = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(reward_type)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(rewards)
    }

    pub async fn claim_reward(&self, reward_id: Uuid, amounts: i32) -> Result<Reward, SqlxError> {
        let reward = sqlx::query_as::<_, Reward>(
            "UPDATE rewards SET claimed_at = $1, updated_at = $2, amounts = $3 WHERE id = $4 AND claimed_at IS NULL RETURNING *",
        )
        .bind(Utc::now())
        .bind(Utc::now())
        .bind(amounts)
        .bind(reward_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(reward)
    }

    pub async fn get_user_reward_stats(&self, user_id: Uuid) -> Result<RewardStats, SqlxError> {
        let stats = sqlx::query_as::<_, RewardStats>(
            "SELECT 
                COALESCE(SUM(points), 0) as total_points,
                COALESCE(SUM(amounts), 0) as total_amounts,
                COUNT(CASE WHEN claimed_at IS NULL THEN 1 END) as unclaimed_rewards,
                COUNT(CASE WHEN claimed_at IS NOT NULL THEN 1 END) as claimed_rewards
            FROM rewards WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(stats)
    }

    pub async fn delete_reward(&self, reward_id: Uuid) -> Result<bool, SqlxError> {
        let row = sqlx::query("DELETE FROM rewards WHERE id = $1")
            .bind(reward_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_reward_points(
        &self,
        reward_id: Uuid,
        points: i32,
        amounts: i32,
    ) -> Result<Reward, SqlxError> {
        let reward = sqlx::query_as::<_, Reward>(
            "UPDATE rewards SET points = $1, amounts = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        )
        .bind(points)
        .bind(amounts)
        .bind(Utc::now())
        .bind(reward_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(reward)
    }

    pub async fn get_all_rewards(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Reward>, SqlxError> {
        let rewards = sqlx::query_as::<_, Reward>(
            "SELECT * FROM rewards ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(rewards)
    }

    pub async fn get_rewards_count_by_user(&self, user_id: Uuid) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM rewards WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(count)
    }

    pub async fn get_unclaimed_rewards_count_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM rewards WHERE user_id = $1 AND claimed_at IS NULL",
        )
        .bind(user_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(count)
    }
}
