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

    pub async fn update_project_step_2(
        &self,
        id: Uuid,
        details: String,
        personnel_cost: i32,
        equipment_cost: Option<i32>,
        materials_cost: Option<i32>,
        overhead_cost: Option<i32>,
        other_cost: i32,
        tags: Vec<String>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE project SET details = $1, personnel_cost = $2, equipment_cost = $3, materials_cost = $4, overhead_cost = $5, other_cost = $6, tags = $7 WHERE id = $8")
            .bind(details)
            .bind(personnel_cost)
            .bind(equipment_cost)
            .bind(materials_cost)
            .bind(overhead_cost)
            .bind(other_cost)
            .bind(tags)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn create_team_member(
        &self,
        project_id: Uuid,
        name: String,
        role: String,
        bio: String,
        linkedin: String,
        twitter: String,
        github: String,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO team_member (project_id, name, role, bio, linkedin, twitter, github) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(project_id)
            .bind(name)
            .bind(role)
            .bind(bio)
            .bind(linkedin)
            .bind(twitter)
            .bind(github)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn delete_team_members(&self, project_id: Uuid) -> Result<bool, SqlxError> {
        let _ = sqlx::query("DELETE FROM team_member WHERE project_id = $1")
            .bind(project_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(true)
    }

    pub async fn create_milestone(
        &self,
        project_id: Uuid,
        number: i16,
        title: String,
        description: String,
        funding_amount: i32,
        days_after_start: i32,
        days_of_prediction: i32,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO milestone (project_id, number, title, description, funding_amount, days_after_start, days_of_prediction) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(project_id)
            .bind(number)
            .bind(title)
            .bind(description)
            .bind(funding_amount)
            .bind(days_after_start)
            .bind(days_of_prediction)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn delete_milestones(&self, project_id: Uuid) -> Result<bool, SqlxError> {
        let _ = sqlx::query("DELETE FROM milestone WHERE project_id = $1")
            .bind(project_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(true)
    }
}
