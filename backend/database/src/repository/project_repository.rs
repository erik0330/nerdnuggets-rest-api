use crate::pool::DatabasePool;
use chrono::Utc;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::{
    models::{Milestone, Project, ProjectItem, TeamMember},
    FeedbackStatus, ProjectStatus,
};
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

    pub async fn create_project(&self, user_id: Uuid, nerd_id: &str) -> Result<Project, SqlxError> {
        let project = sqlx::query_as::<_, Project>(
            "INSERT INTO project (user_id, nerd_id)
            VALUES ($1, $2) RETURNING *",
        )
        .bind(user_id)
        .bind(nerd_id)
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

    pub async fn get_team_members(&self, project_id: Uuid) -> Vec<TeamMember> {
        sqlx::query_as::<_, TeamMember>(
            "SELECT * FROM team_member WHERE project_id = $1 ORDER BY created_at",
        )
        .bind(project_id)
        .fetch_all(self.db_conn.get_pool())
        .await
        .unwrap_or(Vec::new())
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

    pub async fn get_milestones(&self, project_id: Uuid) -> Vec<Milestone> {
        sqlx::query_as::<_, Milestone>(
            "SELECT * FROM milestone WHERE project_id = $1 ORDER BY created_at",
        )
        .bind(project_id)
        .fetch_all(self.db_conn.get_pool())
        .await
        .unwrap_or(Vec::new())
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

    pub async fn submit_project(&self, id: Uuid) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE project SET status = $1 WHERE id = $2")
            .bind(ProjectStatus::PendingReview.to_i16())
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_projects(
        &self,
        title: Option<String>,
        category_id: Option<Uuid>,
        // tab: Option<i16>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<ProjectItem>, SqlxError> {
        let mut filters = Vec::new();
        let mut index = 3;
        let mut query = format!("SELECT p.id, p.nerd_id, p.user_id, p.title, p.description, p.cover_photo, p.category, p.status, p.funding_goal, p.duration, p.tags, p.funding_amount, p.count_contributors, p.created_at, p.updated_at, p.dao_at, p.started_at FROM project p");
        if title.as_ref().map_or(false, |s| !s.is_empty()) {
            filters.push(format!("p.title ILIKE ${index}"));
            index += 1;
        }
        if category_id.is_some() {
            filters.push(format!("${index} = ANY(p.category)"));
        }
        if !filters.is_empty() {
            query = format!("{} WHERE {}", &query, &filters.join(" AND "));
        }
        query = format!("{} ORDER BY p.updated_at DESC LIMIT $1 OFFSET $2", &query);
        let mut query = sqlx::query_as::<_, ProjectItem>(&query)
            .bind(limit.unwrap_or(5))
            .bind(offset.unwrap_or(0));
        if let Some(title) = title.as_ref().filter(|s| !s.is_empty()) {
            query = query.bind(format!("%{}%", title));
        }
        if let Some(category_id) = category_id {
            query = query.bind(category_id)
        }
        let projects = query.fetch_all(self.db_conn.get_pool()).await?;
        Ok(projects)
    }

    pub async fn update_project_status(
        &self,
        id: Uuid,
        status: &ProjectStatus,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE project SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(status.to_i16())
            .bind(Utc::now())
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn create_project_editor(
        &self,
        id: Uuid,
        nerd_id: &str,
        editor_id: Uuid,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query(
            "INSERT INTO project_editor (project_id, nerd_id, user_id) VALUES ($1, $2, $3)",
        )
        .bind(id)
        .bind(nerd_id)
        .bind(editor_id)
        .execute(self.db_conn.get_pool())
        .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_project_editor(
        &self,
        id: Uuid,
        editor_id: Uuid,
        status: &FeedbackStatus,
        feedback: Option<String>,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE project_editor SET status = $1, feedback = $2, updated_at = $3 WHERE project_id = $4 AND user_id = $5")
            .bind(status.to_i16())
            .bind(feedback)
            .bind(Utc::now())
            .bind(id)
            .bind(editor_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }
}
