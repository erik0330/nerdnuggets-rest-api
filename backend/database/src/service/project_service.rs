use crate::{pool::DatabasePool, ProjectRepository, UserRepository, UtilRepository};
use chrono::{Datelike, Utc};
use std::sync::Arc;
use types::{
    dto::{
        ProjectUpdateStep1Request, ProjectUpdateStep2Request, ProjectUpdateStep3Request,
        UpdateMilestoneRequest,
    },
    error::{ApiError, DbError, UserError},
    models::{Milestone, Project, ProjectIds, ProjectInfo, ProjectItemInfo},
    FeedbackStatus, ProjectStatus, UserRoleType,
};
use utils::commons::{generate_random_number, uuid_from_str};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectService {
    project_repo: ProjectRepository,
    user_repo: UserRepository,
    util_repo: UtilRepository,
}

impl ProjectService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            project_repo: ProjectRepository::new(db_conn),
            user_repo: UserRepository::new(db_conn),
            util_repo: UtilRepository::new(db_conn),
        }
    }

    pub async fn project_to_info(&self, project: &Project) -> Result<ProjectInfo, ApiError> {
        let user = self
            .user_repo
            .get_by_user_id(project.user_id)
            .await
            .ok_or_else(|| ApiError::UserError(UserError::UserNotFound))?;
        let category = self.util_repo.get_category_by_ids(&project.category).await;
        let team_members = self.project_repo.get_team_members(project.id).await;
        let milestones = self.project_repo.get_milestones(project.id).await;
        Ok(project.to_info(user.to_info(), category, team_members, milestones))
    }

    pub async fn get_project_by_id(&self, id: &str) -> Result<ProjectInfo, ApiError> {
        let project = if let Ok(id) = uuid_from_str(id) {
            self.project_repo
                .get_project_by_id(id)
                .await
                .ok_or_else(|| DbError::Str("Project not found".to_string()))?
        } else if id.starts_with("RP-") {
            self.project_repo
                .get_project_by_nerd_id(id)
                .await
                .ok_or_else(|| DbError::Str("Project not found".to_string()))?
        } else {
            return Err(DbError::Str("Invalid id format".to_string()).into());
        };
        self.project_to_info(&project).await
    }

    pub async fn create_project(&self, user_id: Uuid) -> Result<ProjectInfo, ApiError> {
        let (nerd_id, proposal_id) = loop {
            let year = Utc::now().year();
            let rand = generate_random_number(1000, 9999);
            let nerd_id = format!("RP-{}-{}", year, rand);
            if self.project_repo.check_nerd_id(&nerd_id).await {
                break (nerd_id, year * 1000 + rand as i32);
            }
        };
        let project = self
            .project_repo
            .create_project(user_id, &nerd_id, proposal_id as i64)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;
        self.project_to_info(&project).await
    }

    pub async fn update_project_step_1(
        &self,
        id: &str,
        payload: ProjectUpdateStep1Request,
    ) -> Result<bool, ApiError> {
        let res = self
            .project_repo
            .update_project_step_1(
                uuid_from_str(id)?,
                payload.manuscript,
                payload.upload_files.unwrap_or_default(),
                payload.cover_photo,
                payload.title,
                payload.description,
                payload.category,
                payload.funding_goal,
                payload.duration,
                payload.youtube_link,
            )
            .await
            .map_err(|_| DbError::Str("Update project failed".to_string()))?;
        Ok(res)
    }

    pub async fn update_project_step_2(
        &self,
        id: &str,
        payload: ProjectUpdateStep2Request,
    ) -> Result<bool, ApiError> {
        let res = self
            .project_repo
            .update_project_step_2(
                uuid_from_str(id)?,
                payload.details,
                payload.personnel_cost,
                payload.equipment_cost,
                payload.materials_cost,
                payload.overhead_cost,
                payload.other_cost,
                payload.tags.unwrap_or_default(),
            )
            .await
            .map_err(|_| DbError::Str("Update project failed".to_string()))?;
        Ok(res)
    }

    pub async fn update_project_step_3(
        &self,
        id: &str,
        payload: ProjectUpdateStep3Request,
    ) -> Result<bool, ApiError> {
        let project_id = uuid_from_str(id)?;
        self.project_repo.delete_team_members(project_id).await.ok();
        self.project_repo.delete_milestones(project_id).await.ok();
        for tm in payload.team_members {
            self.project_repo
                .create_team_member(
                    project_id,
                    tm.name,
                    tm.role,
                    tm.bio,
                    tm.linkedin,
                    tm.twitter,
                    tm.github,
                )
                .await
                .ok();
        }
        for ms in payload.milestones {
            self.project_repo
                .create_milestone(
                    project_id,
                    ms.number,
                    ms.title,
                    ms.description,
                    ms.funding_amount,
                    ms.days_after_start,
                    ms.days_of_prediction,
                )
                .await
                .ok();
        }
        Ok(true)
    }

    pub async fn submit_project(&self, id: &str) -> Result<bool, ApiError> {
        self.project_repo
            .submit_project(uuid_from_str(id)?)
            .await
            .map_err(|_| DbError::Str("Submit project failed".to_string()).into())
    }

    pub async fn get_project_ids(&self) -> Result<Vec<ProjectIds>, ApiError> {
        Ok(self
            .project_repo
            .get_project_ids()
            .await
            .unwrap_or_default())
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
    ) -> Result<Vec<ProjectItemInfo>, ApiError> {
        let status = status.map(|s| ProjectStatus::from(s).to_i16());
        let projects = self
            .project_repo
            .get_projects(
                title,
                status,
                category_id,
                role,
                user_id,
                is_mine,
                is_public,
                offset,
                limit,
            )
            .await
            .map_err(|_| DbError::Str("Get projects failed".to_string()))?;
        let mut project_infos = Vec::new();
        for pro in projects {
            if let Some(user) = self.user_repo.get_by_user_id(pro.user_id).await {
                let category = self.util_repo.get_category_by_ids(&pro.category).await;
                project_infos.push(pro.to_info(user.to_info(), None, category));
            }
        }
        Ok(project_infos)
    }

    pub async fn assign_editor(&self, id: &str, editor_id: Uuid) -> Result<bool, ApiError> {
        let id = uuid_from_str(id)?;
        let editor = self
            .user_repo
            .get_by_user_id(editor_id)
            .await
            .ok_or(DbError::Str("Editor not found".to_string()))?;
        if !editor.roles.contains(&UserRoleType::Editor.to_string()) {
            return Err(DbError::Str("This user has not an editor role".to_string()).into());
        }
        let project = self
            .project_repo
            .get_project_by_id(id)
            .await
            .ok_or(DbError::Str("Project not found".to_string()))?;
        if project.status != ProjectStatus::PendingReview.to_i16() {
            return Err(DbError::Str("Project's status is not PendingReview".to_string()).into());
        }
        if !self
            .project_repo
            .create_project_editor(id, &project.nerd_id, editor_id)
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Can't create a project editor".to_string()).into());
        }
        if !self
            .project_repo
            .update_project_status(id, &ProjectStatus::UnderReview)
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str(
                "Can't update the status of the project when assigning an editor".to_string(),
            )
            .into());
        }
        Ok(true)
    }

    pub async fn decide_editor(
        &self,
        id: &str,
        editor_id: Uuid,
        status: FeedbackStatus,
        feedback: Option<String>,
    ) -> Result<bool, ApiError> {
        let id = uuid_from_str(id)?;
        if !self
            .project_repo
            .update_project_editor(id, editor_id, &status, feedback)
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Update project editor failed".to_string()).into());
        }
        let status = match status {
            FeedbackStatus::Accepted => ProjectStatus::ApprovedEditor,
            FeedbackStatus::RevisionRequired => ProjectStatus::RevisionEditor,
            FeedbackStatus::Rejected => ProjectStatus::Rejected,
            FeedbackStatus::Pending => {
                return Err(DbError::Str("Status should not be Pending".to_string()).into());
            }
        };
        if !self
            .project_repo
            .update_project_status(id, &status)
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str(
                "Can't update the status of the project when editor decision".to_string(),
            )
            .into());
        }
        Ok(true)
    }

    pub async fn decide_admin(
        &self,
        id: &str,
        status: FeedbackStatus,
        feedback: Option<String>,
        to_dao: bool,
    ) -> Result<bool, ApiError> {
        let id = uuid_from_str(id)?;
        let (status, dao_at, started_at) = match status {
            FeedbackStatus::Accepted if to_dao => {
                (ProjectStatus::DaoVoting, Some(Utc::now()), None)
            }
            FeedbackStatus::Accepted => (ProjectStatus::Funding, None, Some(Utc::now())),
            FeedbackStatus::RevisionRequired => (ProjectStatus::RevisionAdmin, None, None),
            FeedbackStatus::Rejected => (ProjectStatus::Rejected, None, None),
            FeedbackStatus::Pending => {
                return Err(DbError::Str("Status should not be Pending".to_string()).into());
            }
        };
        if let Ok(project) = self
            .project_repo
            .decide_admin(id, &status, feedback, dao_at, started_at)
            .await
        {
            if project.status == ProjectStatus::DaoVoting.to_i16() {
                if !self
                    .project_repo
                    .create_dao(&project)
                    .await
                    .unwrap_or_default()
                {
                    return Err(DbError::Str("Create dao failed".to_string()).into());
                }
            } else if project.status == ProjectStatus::Funding.to_i16() {
                let milestones = self.project_repo.get_milestones(project.id).await;
                if !milestones.is_empty() {
                    if !self
                        .project_repo
                        .update_milestone_status(milestones[0].id, 1i16)
                        .await
                        .unwrap_or_default()
                    {
                        return Err(DbError::Str(
                            "Update milestone.0 status to 'In Progress' failed".to_string(),
                        )
                        .into());
                    }
                }
            }
        } else {
            return Err(DbError::Str("Admin decision failed".to_string()).into());
        }
        Ok(true)
    }

    pub async fn update_milestone(
        &self,
        id: &str,
        payload: UpdateMilestoneRequest,
    ) -> Result<bool, ApiError> {
        let proof_status = if payload.is_draft { 0 } else { 1 };
        if !self
            .project_repo
            .update_milestone(
                uuid_from_str(id)?,
                payload.description,
                payload.deliverables,
                payload.challenges,
                payload.next_steps,
                payload.file_urls.unwrap_or_default(),
                proof_status,
            )
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Update milestone failed".to_string()).into());
        }
        Ok(true)
    }

    pub async fn get_milestones(&self, id: &str) -> Result<Vec<Milestone>, ApiError> {
        Ok(self.project_repo.get_milestones(uuid_from_str(id)?).await)
    }
}
