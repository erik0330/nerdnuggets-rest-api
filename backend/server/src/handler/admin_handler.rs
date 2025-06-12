use crate::state::AppState;
use axum::{
    extract::{Query, State},
    Extension, Json,
};
use types::{
    dto::{
        AdminAddCategoriesRequest, AdminApproveUserRoleRequest, AdminDeleteCategoryRequest,
        AdminSuspendUserRequest, AdminUpdateCategoryRequest, AdminUpdateUserInfoRequest,
        AdminUserIdRequest, GetUsersOnDashboardOption, StartLimitOption, UserUpdateResponse,
    },
    error::{ApiError, UserError, ValidatedRequest},
    models::{MessageType, NotificationType, User, UserInfo},
    UserRoleType,
};
use url::Url;
use uuid::Uuid;

// pub async fn get_users_on_dashboard(
//     Extension(user): Extension<User>,
//     Extension(role): Extension<UserRoleType>,
//     opts: Option<Query<GetUsersOnDashboardOption>>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<UserDashboardInfo>>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let Query(opts) = opts.unwrap_or_default();
//     let users = state
//         .service
//         .admin
//         .get_users_on_dashboard(
//             user.id,
//             opts.tab,
//             opts.name,
//             opts.user_name,
//             opts.email,
//             opts.noble_id,
//             opts.status,
//             opts.start,
//             opts.limit,
//         )
//         .await?;

//     Ok(Json(users))
// }

// pub async fn get_user_counts_on_dashboard_tab(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<i64>>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     Ok(Json(
//         state
//             .service
//             .admin
//             .get_user_counts_on_dashboard_tab()
//             .await?,
//     ))
// }

// pub async fn remove_avatar(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminUserIdRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     if let Some(user) = state
//         .service
//         .user
//         .find_by_user_id(payload.user_id)
//         .await
//         .ok()
//     {
//         if let Some(avatar) = user.avatar_url {
//             let bucket_name = state.env.aws_bucket_name;
//             let result = state.service.user.update_avatar_url(user.id, None).await?;
//             if let Some(url) = Url::parse(&avatar).ok() {
//                 let key = url.path().trim_start_matches('/').to_string();
//                 state
//                     .s3_client
//                     .delete_object()
//                     .bucket(&bucket_name)
//                     .key(key)
//                     .send()
//                     .await
//                     .ok();
//             }
//             return Ok(Json(UserUpdateResponse {
//                 state: result.state,
//             }));
//         }
//     }
//     Ok(Json(UserUpdateResponse { state: false }))
// }

// pub async fn add_admin(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminUserIdRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let result = state.service.admin.add_admin(payload.user_id).await;
//     if result {
//         state
//             .service
//             .notification
//             .add_notification(
//                 MessageType::One,
//                 NotificationType::ApproveAdminRole,
//                 &payload.user_id,
//                 &None,
//                 &None,
//             )
//             .await?;
//     }
//     Ok(Json(UserUpdateResponse { state: result }))
// }

// pub async fn remove_admin(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminUserIdRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     Ok(Json(UserUpdateResponse {
//         state: state.service.admin.remove_admin(payload.user_id).await,
//     }))
// }

// pub async fn suspend_user(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminSuspendUserRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     Ok(Json(UserUpdateResponse {
//         state: state
//             .service
//             .admin
//             .suspend_user(payload.user_id, payload.status.unwrap_or_default())
//             .await,
//     }))
// }

// pub async fn approve_user_role(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminApproveUserRoleRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }

//     let result = state
//         .service
//         .admin
//         .approve_user_role(
//             payload.user_id,
//             payload.tab,
//             payload.status.unwrap_or_default(),
//         )
//         .await;

//     if payload.status.unwrap_or_default() && result {
//         let notification_type = match payload.tab {
//             Some(1) => NotificationType::ApproveAuthorRole,
//             Some(2) => NotificationType::ApproveEditorRole,
//             Some(3) => NotificationType::ApproveReviewerRole,
//             Some(4) => NotificationType::ApproveAdminRole,
//             Some(6) => NotificationType::ApproveCopyEditorRole,
//             _ => return Ok(Json(UserUpdateResponse { state: result })),
//         };

//         state
//             .service
//             .notification
//             .add_notification(
//                 MessageType::One,
//                 notification_type,
//                 &payload.user_id,
//                 &None,
//                 &None,
//             )
//             .await?;
//     }
//     Ok(Json(UserUpdateResponse { state: result }))
// }

// pub async fn update_user_info(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminUpdateUserInfoRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     Ok(Json(UserUpdateResponse {
//         state: state
//             .service
//             .admin
//             .update_user_info(
//                 payload.user_id,
//                 payload.first_name,
//                 payload.middle_name,
//                 payload.second_name,
//                 payload.about_me,
//             )
//             .await,
//     }))
// }

// pub async fn get_reported_members(
//     Extension(me): Extension<User>,
//     Extension(role): Extension<UserRoleType>,
//     opts: Option<Query<StartLimitOption>>,
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<UserInfo>>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let Query(opts) = opts.unwrap_or_default();
//     let result = state
//         .service
//         .admin
//         .get_reported_members(me.id, opts.start.unwrap_or(0), opts.limit.unwrap_or(10))
//         .await?;
//     Ok(Json(result))
// }

// pub async fn unreport_user(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminUserIdRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let result = state.service.admin.unreport_user(payload.user_id).await?;
//     Ok(Json(UserUpdateResponse { state: result }))
// }

// pub async fn delete_user(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminUserIdRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let result = state.service.admin.delete_user(payload.user_id).await?;
//     Ok(Json(UserUpdateResponse { state: result }))
// }

// pub async fn add_categories(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminAddCategoriesRequest>,
// ) -> Result<Json<Vec<Uuid>>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let ids = state
//         .service
//         .util
//         .insert_categories(payload.category_names)
//         .await;
//     Ok(Json(ids))
// }

// pub async fn update_category(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminUpdateCategoryRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let result = state
//         .service
//         .util
//         .update_category(payload.id, &payload.category_name, payload.is_available)
//         .await?;
//     Ok(Json(UserUpdateResponse { state: result }))
// }

// pub async fn delete_category(
//     Extension(role): Extension<UserRoleType>,
//     State(state): State<AppState>,
//     ValidatedRequest(payload): ValidatedRequest<AdminDeleteCategoryRequest>,
// ) -> Result<Json<UserUpdateResponse>, ApiError> {
//     if role != UserRoleType::SuperAdmin {
//         return Err(UserError::RoleNotAllowed)?;
//     }
//     let result = state.service.util.delete_category(payload.id).await?;
//     Ok(Json(UserUpdateResponse { state: result }))
// }
