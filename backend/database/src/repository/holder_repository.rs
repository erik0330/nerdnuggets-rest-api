use crate::pool::DatabasePool;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::models::Holder;

#[derive(Clone)]
pub struct HolderRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl HolderRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn get_holders(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Holder>, SqlxError> {
        let holders = sqlx::query_as::<_, Holder>(
            "SELECT * FROM holder ORDER BY total_coins DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit.unwrap_or(100))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(holders)
    }

    pub async fn get_holders_count(&self) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM holder")
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(count)
    }

    pub async fn get_holder_by_telegram_id(&self, telegram_id: &str) -> Option<Holder> {
        sqlx::query_as::<_, Holder>("SELECT * FROM holder WHERE telegram_id = $1")
            .bind(telegram_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn update_holder_claim(
        &self,
        telegram_id: &str,
        wallet_address: &str,
    ) -> Result<bool, SqlxError> {
        // Use UPDATE with WHERE conditions to atomically check and update
        // This prevents race conditions - only updates if not already claimed and has points
        let result = sqlx::query(
            "UPDATE holder 
             SET wallet_address = $1, claimed_at = NOW(), updated_at = NOW() 
             WHERE telegram_id = $2 
               AND claimed_at IS NULL 
               AND current_coins > 0",
        )
        .bind(wallet_address)
        .bind(telegram_id)
        .execute(self.db_conn.get_pool())
        .await?;
        // Return true if a row was actually updated, false otherwise
        Ok(result.rows_affected() > 0)
    }
}
