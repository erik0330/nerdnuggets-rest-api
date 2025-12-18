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
}
