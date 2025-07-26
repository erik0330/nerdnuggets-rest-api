use crate::pool::DatabasePool;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::{
    dto::ProjectStatusCount,
    models::{
        CompletedDao, Dao, DaoVote, Milestone, Prediction, PredictionStatus, Project,
        ProjectComment, ProjectIds, ProjectItem, TeamMember,
    },
    FeedbackStatus, ProjectStatus, UserRoleType,
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

    pub async fn get_project_by_nerd_id(&self, nerd_id: &str) -> Option<Project> {
        sqlx::query_as::<_, Project>("SELECT * FROM project WHERE nerd_id = $1")
            .bind(nerd_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_project_by_proposal_id(&self, proposal_id: i64) -> Option<Project> {
        sqlx::query_as::<_, Project>("SELECT * FROM project WHERE proposal_id = $1")
            .bind(proposal_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn check_project_nerd_id(&self, nerd_id: &str) -> bool {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM project WHERE nerd_id = $1",
            nerd_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await
        .map(|row| row.count.unwrap_or(0))
        .unwrap_or(0);
        count == 0
    }

    pub async fn check_prediction_nerd_id(&self, nerd_id: &str) -> bool {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM prediction WHERE nerd_id = $1",
            nerd_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await
        .map(|row| row.count.unwrap_or(0))
        .unwrap_or(0);
        count == 0
    }

    pub async fn create_project(
        &self,
        user_id: Uuid,
        nerd_id: &str,
        proposal_id: i64,
    ) -> Result<Project, SqlxError> {
        let project = sqlx::query_as::<_, Project>(
            "INSERT INTO project (user_id, nerd_id, proposal_id)
            VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(nerd_id)
        .bind(proposal_id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(project)
    }

    pub async fn delete_project(&self, id: Uuid) -> Result<bool, SqlxError> {
        let row = sqlx::query("DELETE FROM project WHERE id = $1")
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
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
            "SELECT * FROM milestone WHERE project_id = $1 ORDER BY number",
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

    pub async fn get_project_ids(&self) -> Result<Vec<ProjectIds>, SqlxError> {
        let ids = sqlx::query_as::<_, ProjectIds>("SELECT id, nerd_id FROM project")
            .fetch_all(self.db_conn.get_pool())
            .await?;
        Ok(ids)
    }

    pub async fn get_projects(
        &self,
        title: Option<String>,
        status: Option<i16>,
        category_id: Option<Uuid>,
        role: Option<String>,
        user_id: Option<Uuid>,
        is_mine: Option<bool>,
        is_public: Option<bool>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<ProjectItem>, SqlxError> {
        let mut filters = Vec::new();
        let mut index = 3;
        let mut query = format!("SELECT p.id, p.nerd_id, p.proposal_id, p.user_id, p.title, p.description, p.cover_photo, p.category, p.status, p.funding_goal, p.duration, p.tags, p.arweave_tx_id, p.funding_amount, p.count_contributors, p.created_at, p.updated_at, p.dao_at, p.started_at FROM project p");
        if title.as_ref().map_or(false, |s| !s.is_empty()) {
            filters.push(format!("p.title ILIKE ${index}"));
            index += 1;
        }
        if is_mine.unwrap_or_default() {
            if user_id.is_some() {
                filters.push(format!("p.user_id = ${index}"));
                index += 1;
                if status.is_some() {
                    filters.push(format!("p.status = ${index}"));
                    index += 1;
                }
            } else {
                return Ok(Vec::new());
            }
        } else {
            if is_public.unwrap_or_default() {
                if status.is_some() {
                    filters.push(format!("p.status = ${index}"));
                    index += 1;
                } else {
                    filters.push(format!(
                        "(p.status = {} OR p.status = {})",
                        ProjectStatus::Funding.to_i16(),
                        ProjectStatus::Completed.to_i16()
                    ));
                }
            } else {
                match role.clone() {
                    Some(r) if r == UserRoleType::Admin.to_string() => {
                        if status.is_some() {
                            filters.push(format!("p.status = ${index}"));
                            index += 1;
                        } else {
                            filters
                                .push(format!("p.status != {}", ProjectStatus::Creating.to_i16()));
                        }
                    }
                    Some(r) if r == UserRoleType::Editor.to_string() => {
                        if user_id.is_some() {
                            query = format!("{} JOIN project_editor pe ON p.id = pe.project_id AND pe.user_id = ${index} ", &query);
                            index += 1;
                            if let Some(s) = status {
                                match ProjectStatus::from(s) {
                                    ProjectStatus::UnderReview => {
                                        filters.push(format!(
                                            "p.status = {}",
                                            ProjectStatus::UnderReview.to_i16()
                                        ));
                                    }
                                    _ => {
                                        filters.push(format!(
                                            "p.status > {}",
                                            ProjectStatus::UnderReview.to_i16()
                                        ));
                                    }
                                }
                            }
                        } else {
                            return Ok(Vec::new());
                        }
                    }
                    _ => {}
                }
            }
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
        if is_mine.unwrap_or_default() {
            if let Some(user_id) = user_id {
                query = query.bind(user_id);
                if let Some(s) = status {
                    query = query.bind(s);
                }
            }
        } else {
            if is_public.unwrap_or_default() {
                if let Some(s) = status {
                    query = query.bind(s);
                }
            } else {
                match role {
                    Some(r) if r == UserRoleType::Admin.to_string() => {
                        if let Some(s) = status {
                            query = query.bind(s);
                        }
                    }
                    Some(r) if r == UserRoleType::Editor.to_string() => {
                        if let Some(user_id) = user_id {
                            query = query.bind(user_id);
                        }
                    }
                    _ => {}
                }
            }
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

    pub async fn decide_admin(
        &self,
        id: Uuid,
        status: &ProjectStatus,
        feedback: Option<String>,
        dao_at: Option<DateTime<Utc>>,
        started_at: Option<DateTime<Utc>>,
    ) -> Result<Project, SqlxError> {
        let project = sqlx::query_as::<_, Project>(
            "UPDATE project SET status = $1, feedback = $2, updated_at = $3, dao_at = $4, started_at = $5 WHERE id = $6 RETURNING *",
        )
        .bind(status.to_i16())
        .bind(feedback)
        .bind(Utc::now())
        .bind(dao_at)
        .bind(started_at)
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(project)
    }

    pub async fn create_dao(&self, project: &Project) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO dao (project_id, nerd_id, proposal_id, user_id, title, description, funding_goal) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(project.id)
            .bind(&project.nerd_id)
            .bind(project.proposal_id)
            .bind(project.user_id)
            .bind(&project.title)
            .bind(&project.description)
            .bind(project.funding_goal)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_milestone_status(&self, id: Uuid, status: i16) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE milestone SET status = $1 WHERE id = $2")
            .bind(status)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_milestone(
        &self,
        id: Uuid,
        description: String,
        deliverables: Option<String>,
        challenges: Option<String>,
        next_steps: Option<String>,
        file_urls: Vec<String>,
        proof_status: i16,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE milestone SET description = $1, deliverables = $2, challenges = $3, next_steps = $4, file_urls = $5, proof_status = $6 WHERE id = $7")
            .bind(description)
            .bind(deliverables)
            .bind(challenges)
            .bind(next_steps)
            .bind(file_urls)
            .bind(proof_status)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_project_comments(
        &self,
        id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<ProjectComment>, SqlxError> {
        let project_comments = sqlx::query_as::<_, ProjectComment>(
            "SELECT * FROM project_comment WHERE project_id = $1 ORDER BY updated_at LIMIT $2 OFFSET $3",
        )
        .bind(id)
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(project_comments)
    }

    pub async fn submit_project_comment(
        &self,
        user_id: Uuid,
        project_id: Uuid,
        nerd_id: &str,
        comment: &str,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO project_comment (user_id, project_id, nerd_id, comment) VALUES ($1, $2, $3, $4)")
            .bind(user_id)
            .bind(project_id)
            .bind(nerd_id)
            .bind(comment)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_daos(
        &self,
        title: Option<String>,
        status: Option<i16>,
        user_id: Option<Uuid>,
        is_mine: Option<bool>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Dao>, SqlxError> {
        let mut filters = Vec::new();
        let mut index = 3;
        let mut query = format!("SELECT d.* FROM dao d");
        if title.as_ref().map_or(false, |s| !s.is_empty()) {
            filters.push(format!("d.title ILIKE ${index}"));
            index += 1;
        }
        if is_mine.unwrap_or_default() {
            if user_id.is_some() {
                query = format!(
                    "{} JOIN dao_vote dv ON d.id = dv.dao_id AND dv.user_id = ${index} ",
                    &query
                );
            } else {
                return Ok(Vec::new());
            }
        } else if status.is_some() {
            filters.push(format!("d.status = ${index}"));
        }
        if !filters.is_empty() {
            query = format!("{} WHERE {}", &query, &filters.join(" AND "));
        }
        query = format!("{} ORDER BY d.updated_at DESC LIMIT $1 OFFSET $2", &query);
        let mut query = sqlx::query_as::<_, Dao>(&query)
            .bind(limit.unwrap_or(5))
            .bind(offset.unwrap_or(0));
        if let Some(title) = title.as_ref().filter(|s| !s.is_empty()) {
            query = query.bind(format!("%{}%", title));
        }
        if is_mine.unwrap_or_default() {
            if let Some(user_id) = user_id {
                query = query.bind(user_id);
            }
        } else if let Some(s) = status {
            query = query.bind(s);
        }
        let daos = query.fetch_all(self.db_conn.get_pool()).await?;
        Ok(daos)
    }

    pub async fn get_dao_by_id(&self, id: Uuid) -> Result<Dao, SqlxError> {
        let dao = sqlx::query_as::<_, Dao>("SELECT * FROM dao WHERE id = $1")
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(dao)
    }

    pub async fn get_dao_by_project_id(&self, project_id: Uuid) -> Result<Dao, SqlxError> {
        let dao = sqlx::query_as::<_, Dao>("SELECT * FROM dao WHERE project_id = $1")
            .bind(project_id)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(dao)
    }

    pub async fn get_my_dao_vote(&self, dao_id: Uuid, user_id: Uuid) -> Option<DaoVote> {
        let dao_vote = sqlx::query_as::<_, DaoVote>(
            "SELECT * FROM dao_vote WHERE dao_id = $1 AND user_id = $2",
        )
        .bind(dao_id)
        .bind(user_id)
        .fetch_optional(self.db_conn.get_pool())
        .await
        .unwrap_or_default();
        dao_vote
    }

    pub async fn submit_dao_vote(
        &self,
        dao_id: Uuid,
        project_id: Uuid,
        nerd_id: &str,
        proposal_id: i64,
        user_id: Uuid,
        status: i16,
        weight: f32,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("INSERT INTO dao_vote (dao_id, project_id, nerd_id, proposal_id, user_id, status, weight) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(dao_id)
            .bind(project_id)
            .bind(nerd_id)
            .bind(proposal_id)
            .bind(user_id)
            .bind(status)
            .bind(weight)
            .execute(self.db_conn.get_pool())
            .await?;
        let res = row.rows_affected() == 1;
        if res {
            let (count_for, count_against, amount_for, amount_against) = if status == 1i16 {
                (1i32, 0i32, weight, 0f32)
            } else {
                (0i32, 1i32, 0f32, weight)
            };
            let _ = sqlx::query("UPDATE dao SET count_for = count_for + $1, count_against = count_against + $2, amount_for = amount_for + $3, amount_against = amount_against + $4 WHERE id = $5")
                .bind(count_for)
                .bind(count_against)
                .bind(amount_for)
                .bind(amount_against)
                .bind(dao_id)
                .execute(self.db_conn.get_pool())
                .await?;
        }
        Ok(res)
    }

    pub async fn get_completed_daos(
        &self,
        dao_duration: Duration,
    ) -> Result<Vec<CompletedDao>, SqlxError> {
        let dao_duration = dao_duration
            .checked_add(&Duration::minutes(1))
            .unwrap_or_default();
        let daos = sqlx::query_as::<_, CompletedDao>(
            "SELECT id, proposal_id, created_at FROM dao WHERE status = 0 AND created_at <= $1 ORDER BY created_at LIMIT 3",
        )
        .bind(
            Utc::now()
                .checked_sub_signed(dao_duration)
                .unwrap_or_default(),
        )
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(daos)
    }

    pub async fn finish_dao(&self, dao_id: Uuid, status: i16) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE dao SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(status)
            .bind(Utc::now())
            .bind(dao_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn donate_milestone(
        &self,
        user_id: Uuid,
        project_id: Uuid,
        proposal_id: i64,
        number: i16,
        amount: i32,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE milestone SET amount = amount + $1, count_contributors = count_contributors + 1, updated_at = $2 WHERE (status = 0 OR status = 1) AND number = $3 AND project_id = $4")
            .bind(amount)
            .bind(Utc::now())
            .bind(number)
            .bind(project_id)
            .execute(self.db_conn.get_pool())
            .await?;
        if row.rows_affected() > 0 {
            return Ok(false);
        }
        let row = sqlx::query("INSERT INTO funding (project_id, proposal_id, number, user_id, amount) VALUES ($1, $2, $3, $4, $5)")
            .bind(project_id)
            .bind(proposal_id)
            .bind(number)
            .bind(user_id)
            .bind(amount)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn donate_project(&self, project_id: Uuid, amount: i32) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE project SET funding_amount = funding_amount + $1, count_contributors = count_contributors + 1, updated_at = $2 WHERE id = $3")
            .bind(amount)
            .bind(Utc::now())
            .bind(project_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn create_predictions(
        &self,
        nerd_id: &str,
        contract_id: i64,
        milestone: &Milestone,
        project_id: Uuid,
        project_nerd_id: &str,
        proposal_id: i64,
        project_title: &str,
        user_id: Uuid,
        cover_photo: Option<String>,
        category: Vec<Uuid>,
        tags: Vec<String>,
        started_at: NaiveDate,
        ended_at: NaiveDate,
    ) -> Result<Prediction, SqlxError> {
        let prediction = sqlx::query_as::<_, Prediction>("INSERT INTO prediction (nerd_id, contract_id, status, milestone_id, number, title, description, funding_amount, project_id, project_nerd_id, proposal_id, project_title, user_id, cover_photo, category, tags, started_at, ended_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)")
            .bind(nerd_id)
            .bind(contract_id)
            .bind(PredictionStatus::Active)
            .bind(milestone.id)
            .bind(milestone.number)
            .bind(&milestone.title)
            .bind(&milestone.description)
            .bind(milestone.funding_amount)
            .bind(project_id)
            .bind(project_nerd_id)
            .bind(proposal_id)
            .bind(project_title)
            .bind(user_id)
            .bind(cover_photo)
            .bind(category)
            .bind(tags)
            .bind(started_at)
            .bind(ended_at)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(prediction)
    }

    pub async fn get_similar_projects(
        &self,
        project: &Project,
        limit: Option<i32>,
    ) -> Result<Vec<ProjectItem>, SqlxError> {
        let statuses = vec![
            ProjectStatus::DaoVoting as i16,
            ProjectStatus::Funding as i16,
            ProjectStatus::Completed as i16,
        ];
        // Find similar projects based on category, tags, and status
        // We'll look for projects that are public and have similar characteristics
        let similar_projects = sqlx::query_as::<_, ProjectItem>(
            "
            SELECT p.* FROM project p
            WHERE p.id != $1 
            AND p.status = ANY($6)
            AND (
                -- Check for category overlap
                EXISTS (
                    SELECT 1 FROM unnest(p.category) cat1
                    WHERE EXISTS (
                        SELECT 1 FROM unnest($2) cat2
                        WHERE cat1 = cat2
                    )
                )
                OR
                -- Check for tag overlap
                (
                    p.tags IS NOT NULL 
                    AND $3 IS NOT NULL 
                    AND EXISTS (
                        SELECT 1 FROM unnest(p.tags) tag1
                        WHERE EXISTS (
                            SELECT 1 FROM unnest($3) tag2
                            WHERE tag1 ILIKE '%' || tag2 || '%' OR tag2 ILIKE '%' || tag1 || '%'
                        )
                    )
                )
                OR
                -- Check for similar funding goals (within 50% range)
                (
                    p.funding_goal IS NOT NULL 
                    AND $4 IS NOT NULL
                    AND p.funding_goal BETWEEN $4 * 0.5 AND $4 * 1.5
                )
            )
            ORDER BY 
                -- Priority: category match (3 points), tag match (2 points), funding similarity (1 point)
                CASE WHEN EXISTS (
                    SELECT 1 FROM unnest(p.category) cat1
                    WHERE EXISTS (
                        SELECT 1 FROM unnest($2) cat2
                        WHERE cat1 = cat2
                    )
                ) THEN 3 ELSE 0 END +
                CASE WHEN p.tags IS NOT NULL AND $3 IS NOT NULL AND EXISTS (
                    SELECT 1 FROM unnest(p.tags) tag1
                    WHERE EXISTS (
                        SELECT 1 FROM unnest($3) tag2
                        WHERE tag1 ILIKE '%' || tag2 || '%' OR tag2 ILIKE '%' || tag1 || '%'
                    )
                ) THEN 2 ELSE 0 END +
                CASE WHEN p.funding_goal IS NOT NULL AND $4 IS NOT NULL AND p.funding_goal BETWEEN $4 * 0.5 AND $4 * 1.5 THEN 1 ELSE 0 END DESC,
                p.created_at DESC
            LIMIT $5
            ",
        )
        .bind(project.id)
        .bind(&project.category)
        .bind(&project.tags)
        .bind(project.funding_goal)
        .bind(limit.unwrap_or(3))
        .bind(&statuses)
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(similar_projects)
    }

    pub async fn update_project_arweave_tx_id(
        &self,
        id: Uuid,
        arweave_tx_id: &str,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE project SET arweave_tx_id = $1 WHERE id = $2")
            .bind(arweave_tx_id)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn update_milestone_arweave_tx_id(
        &self,
        id: Uuid,
        arweave_tx_id: &str,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE milestone SET arweave_tx_id = $1 WHERE id = $2")
            .bind(arweave_tx_id)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_project_counts_by_status(&self) -> Result<Vec<ProjectStatusCount>, SqlxError> {
        let query =
            "SELECT p.status, COUNT(*) as count FROM project p GROUP BY p.status ORDER BY p.status";
        let counts = sqlx::query_as::<_, ProjectStatusCount>(query)
            .fetch_all(self.db_conn.get_pool())
            .await?;
        Ok(counts)
    }
}
