use crate::{pool::DatabasePool, ProjectRepository, UserRepository};
use std::sync::Arc;
use types::{
    dto::ProjectUpdateStep1Request,
    error::{ApiError, DbError, UserError},
    models::{Project, ProjectInfo},
};
use utils::commons::uuid_from_str;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectService {
    project_repo: ProjectRepository,
    user_repo: UserRepository,
}

impl ProjectService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            project_repo: ProjectRepository::new(db_conn),
            user_repo: UserRepository::new(db_conn),
        }
    }

    pub async fn project_to_info(&self, project: &Project) -> Result<ProjectInfo, ApiError> {
        let user = self
            .user_repo
            .get_by_user_id(project.user_id)
            .await
            .ok_or_else(|| ApiError::UserError(UserError::UserNotFound))?;
        Ok(project.to_info(user.to_info(), Vec::new(), Vec::new()))
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
}
