use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetUsersOnDashboardOption {
    pub tab: Option<u16>, //0: All User, 1: Author, 2: Editor, 3: Reviewer, 4: Admin, 6: Copy-Editor
    pub name: Option<String>, //first_name+middle_name+second_name
    pub user_name: Option<String>,
    pub noble_id: Option<String>,
    pub email: Option<String>,
    pub status: Option<bool>,
    pub start: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AdminUserIdRequest {
    pub user_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AdminSuspendUserRequest {
    pub user_id: Uuid,
    pub status: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AdminApproveUserRoleRequest {
    pub user_id: Uuid,
    pub tab: Option<u16>, //0: All User, 1: Author, 2: Editor, 3: Reviewer, 4: Admin, 6: Copy-Editor
    pub status: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AdminUpdateUserInfoRequest {
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub second_name: Option<String>,
    pub about_me: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct StartLimitOption {
    pub start: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AdminDeleteUnreportPostRequest {
    pub post_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct AdminDeleteCategoryRequest {
    pub id: Uuid,
}
