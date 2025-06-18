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

    pub async fn update_project_step_1(
        &self,
        id: Uuid,
        manuscript: Option<String>,
        upload_files: Vec<String>,
        cover_photo: Option<String>,
        title: String,
        description: String,
        category: Vec<Uuid>,
        funding_goal: i32,
        duration: i32,
        youtube_link: Option<String>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE project SET manuscript = $1, upload_files = $2, cover_photo = $3, title = $4, description = $5, category = $6, funding_goal = $7, duration = $8, youtube_link = $9 WHERE id = $10")
            .bind(manuscript)
            .bind(upload_files)
            .bind(cover_photo)
            .bind(title)
            .bind(description)
            .bind(category)
            .bind(funding_goal)
            .bind(duration)
            .bind(youtube_link)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }
}
