use crate::{pool::DatabasePool, BountyRepository, UserRepository, UtilRepository};
use chrono::{Datelike, NaiveDate, Utc};
use std::sync::Arc;
use types::{
    dto::BountyCreateRequest,
    error::{ApiError, DbError, UserError},
    models::{Bounty, BountyInfo},
};
use utils::commons::{generate_random_number, uuid_from_str};
use uuid::Uuid;

#[derive(Clone)]
pub struct BountyService {
    bounty_repo: BountyRepository,
    user_repo: UserRepository,
    util_repo: UtilRepository,
}

impl BountyService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            bounty_repo: BountyRepository::new(db_conn),
            user_repo: UserRepository::new(db_conn),
            util_repo: UtilRepository::new(db_conn),
        }
    }

    pub async fn bounty_to_info(&self, bounty: &Bounty) -> Result<BountyInfo, ApiError> {
        let user = self
            .user_repo
            .get_user_by_id(bounty.user_id)
            .await
            .ok_or_else(|| ApiError::UserError(UserError::UserNotFound))?;
        let category = self.util_repo.get_category_by_id(bounty.category).await;
        let milestones = self.bounty_repo.get_milestones(bounty.id).await;
        Ok(bounty.to_info(user.to_info(), category, milestones))
    }

    pub async fn get_bounty_by_id(&self, id: &str) -> Result<BountyInfo, ApiError> {
        let bounty = if let Ok(id) = uuid_from_str(id) {
            self.bounty_repo
                .get_bounty_by_id(id)
                .await
                .ok_or_else(|| DbError::Str("Bounty not found".to_string()))?
        } else if id.starts_with("BT-") {
            self.bounty_repo
                .get_bounty_by_nerd_id(id)
                .await
                .ok_or_else(|| DbError::Str("Bounty not found".to_string()))?
        } else {
            return Err(DbError::Str("Invalid id format".to_string()).into());
        };
        self.bounty_to_info(&bounty).await
    }

    pub async fn create_bounty(
        &self,
        user_id: Uuid,
        payload: BountyCreateRequest,
    ) -> Result<BountyInfo, ApiError> {
        let (nerd_id, contract_id) = loop {
            let year = Utc::now().year();
            let rand = generate_random_number(1000, 9999);
            let nerd_id = format!("BT-{}-{}", year, rand);
            if self.bounty_repo.check_nerd_id(&nerd_id).await {
                break (nerd_id, year * 10000 + rand as i32);
            }
        };
        let deadline = NaiveDate::parse_from_str(&payload.deadline, "%m/%d/%Y")
            .map_err(|err| DbError::Str(err.to_string()))?;
        let bounty = self
            .bounty_repo
            .create_bounty(
                user_id,
                &nerd_id,
                contract_id as i64,
                payload.title,
                payload.description,
                payload.upload_file,
                payload.category,
                payload.difficulty,
                payload.tags,
                payload.reward_amount,
                payload.reward_currency,
                deadline,
                payload.requirements,
                payload.deliverables,
                payload.evaluation_criteria,
                payload.by_milestone,
            )
            .await
            .map_err(|err| DbError::Str(err.to_string()))?;
        if payload.by_milestone {
            let mut number = 1;
            for milestone in payload.milestones.unwrap_or_default() {
                match self
                    .bounty_repo
                    .create_milestone(
                        bounty.id,
                        number,
                        milestone.title,
                        milestone.description,
                        milestone.reward_amount,
                        milestone.timeline,
                        milestone.requirements.unwrap_or_default(),
                        milestone.deliverables.unwrap_or_default(),
                    )
                    .await
                {
                    Ok(f) if f => number += 1,
                    Ok(f) => println!("{f}"),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        self.bounty_to_info(&bounty).await
    }

    //     pub async fn update_bounty_step_3(
    //         &self,
    //         id: &str,
    //         payload: BountyUpdateStep3Request,
    //     ) -> Result<bool, ApiError> {
    //         let bounty_id = uuid_from_str(id)?;
    //         self.bounty_repo.delete_team_members(bounty_id).await.ok();
    //         self.bounty_repo.delete_milestones(bounty_id).await.ok();
    //         for tm in payload.team_members {
    //             self.bounty_repo
    //                 .create_team_member(
    //                     bounty_id,
    //                     tm.name,
    //                     tm.role,
    //                     tm.bio,
    //                     tm.linkedin,
    //                     tm.twitter,
    //                     tm.github,
    //                 )
    //                 .await
    //                 .ok();
    //         }
    //         for ms in payload.milestones {
    //             self.bounty_repo
    //                 .create_milestone(
    //                     bounty_id,
    //                     ms.number,
    //                     ms.title,
    //                     ms.description,
    //                     ms.funding_amount,
    //                     ms.days_after_start,
    //                     ms.days_of_prediction,
    //                 )
    //                 .await
    //                 .ok();
    //         }
    //         Ok(true)
    //     }

    //     pub async fn submit_bounty(&self, id: &str) -> Result<bool, ApiError> {
    //         self.bounty_repo
    //             .submit_bounty(uuid_from_str(id)?)
    //             .await
    //             .map_err(|_| DbError::Str("Submit bounty failed".to_string()).into())
    //     }

    //     pub async fn get_bounty_ids(&self) -> Result<Vec<BountyIds>, ApiError> {
    //         Ok(self.bounty_repo.get_bounty_ids().await.unwrap_or_default())
    //     }

    //     pub async fn get_bountys(
    //         &self,
    //         title: Option<String>,
    //         status: Option<i16>,
    //         category_id: Option<Uuid>,
    //         role: Option<String>,
    //         user_id: Option<Uuid>,
    //         is_mine: Option<bool>,
    //         is_public: Option<bool>,
    //         offset: Option<i32>,
    //         limit: Option<i32>,
    //     ) -> Result<Vec<BountyItemInfo>, ApiError> {
    //         let status = status.map(|s| BountyStatus::from(s).to_i16());
    //         let bountys = self
    //             .bounty_repo
    //             .get_bountys(
    //                 title,
    //                 status,
    //                 category_id,
    //                 role,
    //                 user_id,
    //                 is_mine,
    //                 is_public,
    //                 offset,
    //                 limit,
    //             )
    //             .await
    //             .map_err(|_| DbError::Str("Get bountys failed".to_string()))?;
    //         let mut bounty_infos = Vec::new();
    //         for pro in bountys {
    //             if let Some(user) = self.user_repo.get_user_by_id(pro.user_id).await {
    //                 let category = self.util_repo.get_category_by_ids(&pro.category).await;
    //                 bounty_infos.push(pro.to_info(user.to_info(), None, category));
    //             }
    //         }
    //         Ok(bounty_infos)
    //     }

    //     pub async fn assign_editor(&self, id: &str, editor_id: Uuid) -> Result<bool, ApiError> {
    //         let id = uuid_from_str(id)?;
    //         let editor = self
    //             .user_repo
    //             .get_user_by_id(editor_id)
    //             .await
    //             .ok_or(DbError::Str("Editor not found".to_string()))?;
    //         if !editor.roles.contains(&UserRoleType::Editor.to_string()) {
    //             return Err(DbError::Str("This user has not an editor role".to_string()).into());
    //         }
    //         let bounty = self
    //             .bounty_repo
    //             .get_bounty_by_id(id)
    //             .await
    //             .ok_or(DbError::Str("Bounty not found".to_string()))?;
    //         if bounty.status != BountyStatus::PendingReview.to_i16() {
    //             return Err(DbError::Str("Bounty's status is not PendingReview".to_string()).into());
    //         }
    //         if !self
    //             .bounty_repo
    //             .create_bounty_editor(id, &bounty.nerd_id, editor_id)
    //             .await
    //             .unwrap_or_default()
    //         {
    //             return Err(DbError::Str("Can't create a bounty editor".to_string()).into());
    //         }
    //         if !self
    //             .bounty_repo
    //             .update_bounty_status(id, &BountyStatus::UnderReview)
    //             .await
    //             .unwrap_or_default()
    //         {
    //             return Err(DbError::Str(
    //                 "Can't update the status of the bounty when assigning an editor".to_string(),
    //             )
    //             .into());
    //         }
    //         Ok(true)
    //     }

    //     pub async fn decide_editor(
    //         &self,
    //         id: &str,
    //         editor_id: Uuid,
    //         status: FeedbackStatus,
    //         feedback: Option<String>,
    //     ) -> Result<bool, ApiError> {
    //         let id = uuid_from_str(id)?;
    //         if !self
    //             .bounty_repo
    //             .update_bounty_editor(id, editor_id, &status, feedback)
    //             .await
    //             .unwrap_or_default()
    //         {
    //             return Err(DbError::Str("Update bounty editor failed".to_string()).into());
    //         }
    //         let status = match status {
    //             FeedbackStatus::Accepted => BountyStatus::ApprovedEditor,
    //             FeedbackStatus::RevisionRequired => BountyStatus::RevisionEditor,
    //             FeedbackStatus::Rejected => BountyStatus::Rejected,
    //             FeedbackStatus::Pending => {
    //                 return Err(DbError::Str("Status should not be Pending".to_string()).into());
    //             }
    //         };
    //         if !self
    //             .bounty_repo
    //             .update_bounty_status(id, &status)
    //             .await
    //             .unwrap_or_default()
    //         {
    //             return Err(DbError::Str(
    //                 "Can't update the status of the bounty when editor decision".to_string(),
    //             )
    //             .into());
    //         }
    //         Ok(true)
    //     }

    //     pub async fn decide_admin(
    //         &self,
    //         id: &str,
    //         status: FeedbackStatus,
    //         feedback: Option<String>,
    //         to_dao: bool,
    //     ) -> Result<bool, ApiError> {
    //         let id = uuid_from_str(id)?;
    //         let (status, dao_at, started_at) = match status {
    //             FeedbackStatus::Accepted if to_dao => (BountyStatus::DaoVoting, Some(Utc::now()), None),
    //             FeedbackStatus::Accepted => (BountyStatus::Funding, None, Some(Utc::now())),
    //             FeedbackStatus::RevisionRequired => (BountyStatus::RevisionAdmin, None, None),
    //             FeedbackStatus::Rejected => (BountyStatus::Rejected, None, None),
    //             FeedbackStatus::Pending => {
    //                 return Err(DbError::Str("Status should not be Pending".to_string()).into());
    //             }
    //         };
    //         if let Ok(bounty) = self
    //             .bounty_repo
    //             .decide_admin(id, &status, feedback, dao_at, started_at)
    //             .await
    //         {
    //             if bounty.status == BountyStatus::DaoVoting.to_i16() {
    //                 if !self
    //                     .bounty_repo
    //                     .create_dao(&bounty)
    //                     .await
    //                     .unwrap_or_default()
    //                 {
    //                     return Err(DbError::Str("Create dao failed".to_string()).into());
    //                 }
    //             } else if bounty.status == BountyStatus::Funding.to_i16() {
    //                 let milestones = self.bounty_repo.get_milestones(bounty.id).await;
    //                 if !milestones.is_empty() {
    //                     if !self
    //                         .bounty_repo
    //                         .update_milestone_status(milestones[0].id, 1i16)
    //                         .await
    //                         .unwrap_or_default()
    //                     {
    //                         return Err(DbError::Str(
    //                             "Update milestone.0 status to 'In Progress' failed".to_string(),
    //                         )
    //                         .into());
    //                     }
    //                 }
    //             }
    //         } else {
    //             return Err(DbError::Str("Admin decision failed".to_string()).into());
    //         }
    //         Ok(true)
    //     }

    //     pub async fn update_milestone(
    //         &self,
    //         id: &str,
    //         payload: UpdateMilestoneRequest,
    //     ) -> Result<bool, ApiError> {
    //         let proof_status = if payload.is_draft { 0 } else { 1 };
    //         if !self
    //             .bounty_repo
    //             .update_milestone(
    //                 uuid_from_str(id)?,
    //                 payload.description,
    //                 payload.deliverables,
    //                 payload.challenges,
    //                 payload.next_steps,
    //                 payload.file_urls.unwrap_or_default(),
    //                 proof_status,
    //             )
    //             .await
    //             .unwrap_or_default()
    //         {
    //             return Err(DbError::Str("Update milestone failed".to_string()).into());
    //         }
    //         Ok(true)
    //     }

    //     pub async fn get_milestones(&self, id: &str) -> Result<Vec<Milestone>, ApiError> {
    //         Ok(self.bounty_repo.get_milestones(uuid_from_str(id)?).await)
    //     }

    //     pub async fn get_bounty_comments(
    //         &self,
    //         id: &str,
    //         offset: Option<i32>,
    //         limit: Option<i32>,
    //     ) -> Result<Vec<BountyCommentInfo>, ApiError> {
    //         let bounty_comments = self
    //             .bounty_repo
    //             .get_bounty_comments(uuid_from_str(id)?, offset, limit)
    //             .await
    //             .unwrap_or_default();
    //         let mut pc_infos = Vec::new();
    //         for pc in bounty_comments {
    //             if let Some(user) = self.user_repo.get_user_by_id(pc.user_id).await {
    //                 pc_infos.push(pc.to_info(user.to_info()));
    //             }
    //         }
    //         Ok(pc_infos)
    //     }

    //     pub async fn submit_bounty_comment(
    //         &self,
    //         id: &str,
    //         user_id: Uuid,
    //         comment: &str,
    //     ) -> Result<bool, ApiError> {
    //         let res = if let Some(bounty) = self.bounty_repo.get_bounty_by_id(uuid_from_str(id)?).await
    //         {
    //             self.bounty_repo
    //                 .submit_bounty_comment(user_id, bounty.id, &bounty.nerd_id, comment)
    //                 .await
    //                 .unwrap_or_default()
    //         } else {
    //             false
    //         };
    //         Ok(res)
    //     }

    //     pub async fn get_daos(
    //         &self,
    //         title: Option<String>,
    //         status: Option<i16>,
    //         user_id: Option<Uuid>,
    //         is_mine: Option<bool>,
    //         offset: Option<i32>,
    //         limit: Option<i32>,
    //     ) -> Result<Vec<Dao>, ApiError> {
    //         let daos = self
    //             .bounty_repo
    //             .get_daos(title, status, user_id, is_mine, offset, limit)
    //             .await
    //             .map_err(|_| DbError::Str("Get daos failed".to_string()))?;
    //         Ok(daos)
    //     }

    //     pub async fn get_dao_by_id(&self, id: &str) -> Result<Dao, ApiError> {
    //         let dao = self
    //             .bounty_repo
    //             .get_dao_by_id(uuid_from_str(id)?)
    //             .await
    //             .map_err(|_| DbError::Str("Get dao by id failed".to_string()))?;
    //         Ok(dao)
    //     }

    //     pub async fn get_my_dao_vote(
    //         &self,
    //         id: &str,
    //         user_id: Uuid,
    //     ) -> Result<Option<DaoVote>, ApiError> {
    //         let dao_vote = self
    //             .bounty_repo
    //             .get_my_dao_vote(uuid_from_str(id)?, user_id)
    //             .await;
    //         Ok(dao_vote)
    //     }

    //     pub async fn submit_dao_vote(
    //         &self,
    //         id: &str,
    //         user_id: Uuid,
    //         status: i16,
    //         comment: Option<String>,
    //     ) -> Result<bool, ApiError> {
    //         if status != 1 && status != 2 {
    //             return Err(DbError::Str("Status invalid".to_string()))?;
    //         }
    //         if !self
    //             .bounty_repo
    //             .submit_dao_vote(uuid_from_str(id)?, user_id, status, comment)
    //             .await
    //             .unwrap_or_default()
    //         {
    //             return Err(DbError::Str("Submit vote failed".to_string()).into());
    //         }
    //         Ok(true)
    //     }
}
