use crate::{pool::DatabasePool, ProjectRepository, UserRepository, UtilRepository};
use std::sync::Arc;
use types::{
    dto::{ProjectUpdateStep1Request, ProjectUpdateStep2Request, ProjectUpdateStep3Request},
    error::{ApiError, DbError, UserError},
    models::{Project, ProjectInfo},
};
use utils::commons::uuid_from_str;
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
        let project = self
            .project_repo
            .get_project_by_id(uuid_from_str(id)?)
            .await
            .ok_or_else(|| DbError::SomethingWentWrong("Project not found".to_string()))?;
        self.project_to_info(&project).await
    }

    pub async fn create_project(&self, user_id: Uuid) -> Result<ProjectInfo, ApiError> {
        let project = self
            .project_repo
            .create_project(user_id)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()))?;
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
            .map_err(|_| DbError::SomethingWentWrong("Update project failed".to_string()))?;
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
            .map_err(|_| DbError::SomethingWentWrong("Update project failed".to_string()))?;
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
}
