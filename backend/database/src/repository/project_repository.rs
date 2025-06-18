use crate::pool::DatabasePool;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::models::Project;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl ProjectRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn get_project_by_id(&self, id: Uuid) -> Option<Project> {
        sqlx::query_as::<_, Project>("SELECT * FROM project WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn create_project(&self, user_id: Uuid) -> Result<Project, SqlxError> {
        let project = sqlx::query_as::<_, Project>(
            "INSERT INTO project (user_id)
            VALUES ($1) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(project)
    }
}
