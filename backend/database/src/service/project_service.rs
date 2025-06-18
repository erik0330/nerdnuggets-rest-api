use crate::{pool::DatabasePool, ProjectRepository, UserRepository};
use std::{str::FromStr, sync::Arc};
use types::{
    error::{ApiError, DbError, UserError},
    models::{Project, ProjectInfo},
};
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
        let id = Uuid::from_str(id).map_err(|_| {
            ApiError::DbError(DbError::SomethingWentWrong(
                "Invalid UUID format".to_string(),
            ))
        })?;
        let project = self
            .project_repo
            .get_project_by_id(id)
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
}
