use crate::UtilRepository;
use crate::{pool::DatabasePool, repository::UserRepository};
use std::sync::Arc;
use types::error::ApiError;
use types::models::UserInfo;
use uuid::Uuid;

#[derive(Clone)]
pub struct AdminService {
    user_repo: UserRepository,
    util_repo: UtilRepository,
}

impl AdminService {
    pub fn init(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            util_repo: UtilRepository::new(db_conn),
        }
    }

    // pub async fn get_users_on_dashboard(
    //     &self,
    //     id: Uuid,
    //     tab: Option<u16>, //0: All User, 1: Author, 2: Editor, 3: Reviewer, 4: Admin, 6: Copy-Editor
    //     name: Option<String>, //first_name+middle_name+second_name
    //     user_name: Option<String>,
    //     email: Option<String>,
    //     noble_id: Option<String>,
    //     status: Option<bool>,
    //     start: Option<i32>,
    //     limit: Option<i32>,
    // ) -> Result<Vec<UserDashboardInfo>, ApiError> {
    //     let users: Vec<UserDashboardInfo> = self
    //         .user_repo
    //         .get_users_on_dashboard(
    //             id, tab, name, user_name, noble_id, email, status, start, limit,
    //         )
    //         .await
    //         .into_iter()
    //         .map(|u| u.to_user_dashboard_info())
    //         .collect();
    //     Ok(users)
    // }

    // pub async fn get_user_counts_on_dashboard_tab(&self) -> Result<Vec<i64>, ApiError> {
    //     let counts = self.user_repo.get_user_counts_on_dashboard_tab().await;
    //     Ok(counts)
    // }

    // pub async fn add_admin(&self, id: Uuid) -> bool {
    //     self.user_repo.add_admin(id).await
    // }

    // pub async fn remove_admin(&self, id: Uuid) -> bool {
    //     self.user_repo.remove_admin(id).await
    // }

    // pub async fn suspend_user(&self, id: Uuid, status: bool) -> bool {
    //     self.user_repo.suspend_user(id, status).await
    // }

    // pub async fn approve_user_role(&self, id: Uuid, tab: Option<u16>, status: bool) -> bool {
    //     self.user_repo.approve_user_role(id, tab, status).await
    // }

    // pub async fn update_user_info(
    //     &self,
    //     id: Uuid,
    //     first_name: Option<String>,
    //     middle_name: Option<String>,
    //     second_name: Option<String>,
    //     about_me: Option<String>,
    // ) -> bool {
    //     self.user_repo
    //         .update_user_info(id, first_name, middle_name, second_name, about_me)
    //         .await
    // }

    // pub async fn get_reported_members(
    //     &self,
    //     me_id: Uuid,
    //     start: i32,
    //     limit: i32,
    // ) -> Result<Vec<UserInfo>, ApiError> {
    //     let degrees = self.util_repo.get_degree("", None).await;
    //     let users = self.user_repo.get_reported_members(start, limit).await;
    //     let mut user_infos = Vec::new();
    //     for user in users {
    //         let is_following = self.user_repo.check_user_related(me_id, user.id, 0).await;
    //         let is_follower = self.user_repo.check_user_related(me_id, user.id, 1).await;
    //         let is_blocked = self.user_repo.check_user_related(me_id, user.id, 2).await;
    //         let is_reported = self.user_repo.check_user_related(me_id, user.id, 3).await;
    //         user_infos.push(user.to_user_info(
    //             is_following,
    //             is_follower,
    //             is_blocked,
    //             is_reported,
    //             &degrees,
    //         ));
    //     }
    //     Ok(user_infos)
    // }

    // pub async fn unreport_user(&self, user_id: Uuid) -> Result<bool, ApiError> {
    //     Ok(self.user_repo.unreport_user_by_admin(user_id).await)
    // }

    // pub async fn delete_user(&self, user_id: Uuid) -> Result<bool, ApiError> {
    //     Ok(self.user_repo.delete_user(user_id).await)
    // }
}
