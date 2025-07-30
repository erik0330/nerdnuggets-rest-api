use crate::{pool::DatabasePool, ProjectRepository, UserRepository, UtilRepository};
use chrono::{Datelike, Duration, Utc};
use evm::EVMClient;
use std::sync::Arc;
use types::{
    dto::{
        ProjectCountsResponse, ProjectUpdateStep1Request, ProjectUpdateStep2Request,
        ProjectUpdateStep3Request, UpdateMilestoneRequest,
    },
    error::{ApiError, DbError, UserError},
    models::{
        CompletedDao, DaoInfo, DaoVote, Milestone, Project, ProjectCommentInfo, ProjectIds,
        ProjectInfo, ProjectItemInfo,
    },
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
            .get_user_by_id(project.user_id)
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
        // Increment view count
        let _ = self.project_repo.increment_view_count(project.id).await;
        self.project_to_info(&project).await
    }

    pub async fn get_project_by_id_without_increment(
        &self,
        id: &str,
    ) -> Result<ProjectInfo, ApiError> {
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
            if self.project_repo.check_project_nerd_id(&nerd_id).await {
                break (nerd_id, year * 10000 + rand as i32);
            }
        };
        let project = self
            .project_repo
            .create_project(user_id, &nerd_id, proposal_id as i64)
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;
        self.project_to_info(&project).await
    }

    pub async fn delete_project(&self, id: &str, user_id: Uuid) -> Result<bool, ApiError> {
        let id = uuid_from_str(id)?;
        let project = self
            .project_repo
            .get_project_by_id(id)
            .await
            .ok_or(DbError::Str("Project not found".to_string()))?;
        if project.user_id != user_id {
            return Err(DbError::Str("No permission".to_string()).into());
        }
        let res = self
            .project_repo
            .delete_project(id)
            .await
            .map_err(|_| DbError::Str("Delete project failed".to_string()))?;
        Ok(res)
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
            if let Some(user) = self.user_repo.get_user_by_id(pro.user_id).await {
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
            .get_user_by_id(editor_id)
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
        evm: &EVMClient,
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
                let researcher = self
                    .user_repo
                    .get_user_by_id(project.user_id)
                    .await
                    .ok_or(DbError::Str("Researcher not found".to_string()))?;
                let wallet =
                    researcher
                        .wallet_address
                        .filter(|w| !w.is_empty())
                        .ok_or(DbError::Str(
                            "Can't find the wallet address of the researcher".to_string(),
                        ))?;
                let milestones = self.project_repo.get_milestones(project.id).await;
                let milestone_data = milestones
                    .iter()
                    .map(|m| (m.days_after_start as u64, m.funding_amount as u64))
                    .collect();
                let _transaction_id = evm
                    .create_project(
                        project.proposal_id as u64,
                        &wallet,
                        milestone_data,
                        String::new(),
                    )
                    .await
                    .map_err(|e| DbError::Str(e.to_string()))?;
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
                let mut started_at = Utc::now().date_naive();
                for milestone in milestones {
                    let (nerd_id, contract_id) = loop {
                        let year = Utc::now().year();
                        let rand = generate_random_number(1000, 9999);
                        let nerd_id = format!("PN-{}-{}", year, rand);
                        if self.project_repo.check_prediction_nerd_id(&nerd_id).await {
                            break (nerd_id, year * 10000 + rand as i32);
                        }
                    };
                    let ended_at =
                        Utc::now().date_naive() + Duration::days(milestone.days_after_start as i64);
                    let _ = self
                        .project_repo
                        .create_predictions(
                            &nerd_id,
                            contract_id as i64,
                            &milestone,
                            project.id,
                            &project.nerd_id,
                            project.proposal_id,
                            &project.title.clone().unwrap_or_default(),
                            project.user_id,
                            project.cover_photo.clone(),
                            project.category.clone(),
                            project.tags.clone(),
                            started_at,
                            ended_at,
                        )
                        .await
                        .map_err(|e| DbError::Str(e.to_string()));
                    started_at = ended_at + Duration::days(1);
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

    pub async fn get_project_comments(
        &self,
        id: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<ProjectCommentInfo>, ApiError> {
        let project_comments = self
            .project_repo
            .get_project_comments(uuid_from_str(id)?, offset, limit)
            .await
            .unwrap_or_default();
        let mut pc_infos = Vec::new();
        for pc in project_comments {
            if let Some(user) = self.user_repo.get_user_by_id(pc.user_id).await {
                pc_infos.push(pc.to_info(user.to_info()));
            }
        }
        Ok(pc_infos)
    }

    pub async fn submit_project_comment(
        &self,
        id: &str,
        user_id: Uuid,
        comment: &str,
    ) -> Result<bool, ApiError> {
        let res = if let Some(project) = self
            .project_repo
            .get_project_by_id(uuid_from_str(id)?)
            .await
        {
            self.project_repo
                .submit_project_comment(user_id, project.id, &project.nerd_id, comment)
                .await
                .unwrap_or_default()
        } else {
            false
        };
        Ok(res)
    }

    pub async fn get_daos(
        &self,
        title: Option<String>,
        status: Option<i16>,
        user_id: Option<Uuid>,
        is_mine: Option<bool>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<DaoInfo>, ApiError> {
        let daos = self
            .project_repo
            .get_daos(title, status, user_id, is_mine, offset, limit)
            .await
            .map_err(|_| DbError::Str("Get daos failed".to_string()))?;
        let mut dao_infos = Vec::new();
        for dao in daos {
            if let Some(user) = self.user_repo.get_user_by_id(dao.user_id).await {
                let my_vote = if let Some(user_id) = user_id {
                    self.project_repo
                        .get_my_dao_vote(dao.id, user_id)
                        .await
                        .map(|v| v.my_vote())
                } else {
                    None
                };
                dao_infos.push(dao.to_info(user.to_info(), my_vote));
            }
        }
        Ok(dao_infos)
    }

    pub async fn get_dao_by_id(&self, id: &str, user_id: Uuid) -> Result<DaoInfo, ApiError> {
        let dao = self
            .project_repo
            .get_dao_by_id(uuid_from_str(id)?)
            .await
            .map_err(|_| DbError::Str("Get dao by id failed".to_string()))?;
        if let Some(user) = self.user_repo.get_user_by_id(dao.user_id).await {
            let my_vote = self
                .project_repo
                .get_my_dao_vote(dao.id, user_id)
                .await
                .map(|v| v.my_vote());
            Ok(dao.to_info(user.to_info(), my_vote))
        } else {
            Err(ApiError::UserError(UserError::UserNotFound))
        }
    }

    pub async fn get_my_dao_vote(
        &self,
        id: &str,
        user_id: Uuid,
    ) -> Result<Option<DaoVote>, ApiError> {
        let dao_vote = self
            .project_repo
            .get_my_dao_vote(uuid_from_str(id)?, user_id)
            .await;
        Ok(dao_vote)
    }

    pub async fn submit_dao_vote(
        &self,
        proposal_id: i64,
        wallet: &str,
        support: bool,
        weight: u128,
    ) -> Result<bool, ApiError> {
        let weight = (weight as f64) / 10f64.powi(18);
        let status = if support { 1i16 } else { 2i16 };

        let project = self
            .project_repo
            .get_project_by_proposal_id(proposal_id)
            .await
            .ok_or(DbError::Str("Project not found".to_string()))?;
        let dao = self
            .project_repo
            .get_dao_by_project_id(project.id)
            .await
            .map_err(|_| DbError::Str("Dao not found".to_string()))?;
        let user = self
            .user_repo
            .get_user_by_wallet(wallet)
            .await
            .ok_or(DbError::Str("User not found".to_string()))?;
        if self
            .project_repo
            .get_my_dao_vote(dao.id, user.id)
            .await
            .is_some()
        {
            return Err(DbError::Str("You have already voted for this dao".to_string()).into());
        }

        if !self
            .project_repo
            .submit_dao_vote(
                dao.id,
                project.id,
                &project.nerd_id,
                proposal_id,
                user.id,
                status,
                weight as f32,
            )
            .await
            .unwrap_or_default()
        {
            return Err(DbError::Str("Submit vote failed".to_string()).into());
        }
        Ok(true)
    }

    pub async fn get_completed_daos(
        &self,
        dao_duration: Duration,
    ) -> Result<Vec<CompletedDao>, ApiError> {
        self.project_repo
            .get_completed_daos(dao_duration)
            .await
            .map_err(|_| DbError::Str("Failed to fetch completed daos".to_string()).into())
    }

    pub async fn finished_dao(&self, proposal_id: i64, status: bool) -> Result<bool, ApiError> {
        if let Some(project) = self
            .project_repo
            .get_project_by_proposal_id(proposal_id)
            .await
        {
            if project.status == ProjectStatus::DaoVoting.to_i16() {
                if status {
                    if self
                        .project_repo
                        .update_project_status(project.id, &ProjectStatus::Funding)
                        .await
                        .is_ok()
                    {
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
                        let mut started_at = Utc::now().date_naive();
                        for milestone in milestones {
                            let (nerd_id, contract_id) = loop {
                                let year = Utc::now().year();
                                let rand = generate_random_number(1000, 9999);
                                let nerd_id = format!("PN-{}-{}", year, rand);
                                if self.project_repo.check_prediction_nerd_id(&nerd_id).await {
                                    break (nerd_id, year * 10000 + rand as i32);
                                }
                            };
                            let ended_at = Utc::now().date_naive()
                                + Duration::days(milestone.days_after_start as i64);
                            let _ = self
                                .project_repo
                                .create_predictions(
                                    &nerd_id,
                                    contract_id as i64,
                                    &milestone,
                                    project.id,
                                    &project.nerd_id,
                                    project.proposal_id,
                                    &project.title.clone().unwrap_or_default(),
                                    project.user_id,
                                    project.cover_photo.clone(),
                                    project.category.clone(),
                                    project.tags.clone(),
                                    started_at,
                                    ended_at,
                                )
                                .await
                                .map_err(|e| DbError::Str(e.to_string()));
                            started_at = ended_at + Duration::days(1);
                        }
                    }
                } else {
                    self.project_repo
                        .update_project_status(project.id, &ProjectStatus::Rejected)
                        .await
                        .ok();
                }
            }
            if let Ok(dao) = self.project_repo.get_dao_by_project_id(project.id).await {
                if dao.status == 0 {
                    // active
                    let status = if status { 1i16 } else { 2i16 };
                    self.project_repo.finish_dao(dao.id, status).await.ok();
                }
            }
        }
        Ok(true)
    }

    pub async fn donate_milestone(
        &self,
        proposal_id: i64,
        number: i16,
        wallet: &str,
        amount: u128,
    ) -> Result<bool, ApiError> {
        let amount = (amount as f64) / 10f64.powi(18);
        let number = number + 1;
        let user = self
            .user_repo
            .get_user_by_wallet(wallet)
            .await
            .ok_or(DbError::Str("User not found".to_string()))?;
        let project = self
            .project_repo
            .get_project_by_proposal_id(proposal_id)
            .await
            .ok_or(DbError::Str("Project not found".to_string()))?;
        let milestones = self.project_repo.get_milestones(project.id).await;
        if milestones.len() < number as usize {
            return Err(DbError::Str("Milestone not found".to_string()).into());
        }
        if self
            .project_repo
            .donate_milestone(
                user.id,
                project.id,
                project.proposal_id,
                number,
                amount as i32,
            )
            .await
            .unwrap_or_default()
        {
            self.project_repo
                .donate_project(project.id, amount as i32)
                .await
                .ok();
        }
        Ok(true)
    }

    pub async fn get_similar_projects(
        &self,
        id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<ProjectItemInfo>, ApiError> {
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

        let similar_projects = self
            .project_repo
            .get_similar_projects(&project, limit)
            .await
            .map_err(|_| DbError::Str("Failed to get similar projects".to_string()))?;

        let mut project_infos = Vec::new();
        for pro in similar_projects {
            if let Some(user) = self.user_repo.get_user_by_id(pro.user_id).await {
                let category = self.util_repo.get_category_by_ids(&pro.category).await;
                project_infos.push(pro.to_info(user.to_info(), None, category));
            }
        }

        Ok(project_infos)
    }

    pub async fn update_project_arweave_tx_id(
        &self,
        project_id: Uuid,
        arweave_tx_id: &str,
    ) -> Result<bool, ApiError> {
        self.project_repo
            .update_project_arweave_tx_id(project_id, arweave_tx_id)
            .await
            .map_err(|e| DbError::Str(e.to_string()).into())
    }

    pub async fn update_milestone_arweave_tx_id(
        &self,
        milestone_id: Uuid,
        arweave_tx_id: &str,
    ) -> Result<bool, ApiError> {
        let res = self
            .project_repo
            .update_milestone_arweave_tx_id(milestone_id, arweave_tx_id)
            .await
            .map_err(|_| DbError::Str("Update milestone arweave tx id failed".to_string()))?;
        Ok(res)
    }

    pub async fn get_project_counts_by_status_for_user(
        &self,
        user_id: Option<Uuid>,
    ) -> Result<ProjectCountsResponse, ApiError> {
        let all_counts = self
            .project_repo
            .get_project_counts_by_status()
            .await
            .map_err(|_| DbError::Str("Get project counts by status failed".to_string()))?;

        let my_project_count = if let Some(user_id) = user_id {
            let user_counts = self
                .project_repo
                .get_project_counts_by_status_for_user(user_id)
                .await
                .map_err(|_| {
                    DbError::Str("Get user project counts by status failed".to_string())
                })?;
            user_counts.iter().map(|c| c.count).sum()
        } else {
            0
        };

        let funding_count = all_counts
            .iter()
            .find(|c| c.status == ProjectStatus::Funding.to_i16())
            .map_or(0, |c| c.count);
        let funded_count = all_counts
            .iter()
            .find(|c| c.status == ProjectStatus::Completed.to_i16())
            .map_or(0, |c| c.count);
        let all_count = funding_count + funded_count;

        // For now, set featured and trending to 0 as they're not implemented yet
        let featured_count = 0;
        let trending_count = 0;

        Ok(ProjectCountsResponse {
            all: all_count,
            my_project: my_project_count,
            funding: funding_count,
            featured: featured_count,
            trending: trending_count,
            funded: funded_count,
        })
    }
}
