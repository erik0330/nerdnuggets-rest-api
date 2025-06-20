use crate::{pool::DatabasePool, repository::UserRepository, UtilRepository};
use std::{str::FromStr, sync::Arc};
use types::{
    dto::{UserCheckResponse, UserOnboardingRequest},
    error::{ApiError, DbError, UserError},
    models::User,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    _util_repo: UtilRepository,
}

impl UserService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            _util_repo: UtilRepository::new(db_conn),
        }
    }

    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<User, ApiError> {
        self.user_repo
            .get_by_user_id(user_id)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User, ApiError> {
        self.user_repo
            .find_by_email(email)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn find_by_gmail(&self, gmail: &str) -> Result<User, ApiError> {
        self.user_repo
            .find_by_gmail(gmail)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn find_by_website(&self, web_site: &str) -> Result<User, ApiError> {
        self.user_repo
            .find_by_website(web_site)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn find_by_linkedin(&self, linkedin: &str) -> Result<User, ApiError> {
        self.user_repo
            .find_by_linkedin(linkedin)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    pub async fn find_by_orc_id(&self, orc_id: &str) -> Result<User, ApiError> {
        self.user_repo
            .find_by_orc_id(orc_id)
            .await
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    // pub async fn find_by_google_scholar(&self, google_scholar: &str) -> Result<User, ApiError> {
    //     self.user_repo
    //         .find_by_google_scholar(google_scholar)
    //         .await
    //         .ok_or_else(|| UserError::UserNotFound.into())
    // }

    // pub async fn update_twitter(
    //     &self,
    //     id: Uuid,
    //     twitter_id: Option<String>,
    //     twitter_username: Option<String>,
    // ) -> Result<bool, ApiError> {
    //     self.user_repo
    //         .update_twitter(id, twitter_id, twitter_username)
    //         .await
    //         .map_err(|_| DbError::SomethingWentWrong("Update twitter failed".to_string()).into())
    // }

    // pub async fn create_user_with_twitter(
    //     &self,
    //     twitter_id: &str,
    //     twitter_username: &str,
    // ) -> Result<bool, ApiError> {
    //     let noble_id = self.generate_available_noble_id().await;
    //     self.user_repo
    //         .create_user_with_twitter(twitter_id, twitter_username, &noble_id)
    //         .await
    //         .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    // }

    pub async fn update_gmail(&self, id: Uuid, gmail: Option<String>) -> Result<bool, ApiError> {
        self.user_repo
            .update_gmail(id, gmail)
            .await
            .map_err(|_| DbError::Str("Update gmail failed".to_string()).into())
    }

    pub async fn update_user_onboarding(
        &self,
        id: &str,
        payload: UserOnboardingRequest,
    ) -> Result<User, ApiError> {
        let id = Uuid::from_str(id)
            .map_err(|_| ApiError::DbError(DbError::Str("Invalid UUID format".to_string())))?;
        self.user_repo
            .update_user_onboarding(
                id,
                &payload.name,
                &payload.institution,
                &payload.bio,
                payload.roles,
                payload.interests,
                &payload.wallet_address,
            )
            .await
            .map_err(|_| DbError::Str("Update user onboarding failed".to_string()).into())
    }

    pub async fn create_user_with_google(&self, gmail: &str) -> Result<User, ApiError> {
        self.user_repo
            .create_user_with_google(gmail)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn check_email(&self, email: &str) -> Result<UserCheckResponse, ApiError> {
        Ok(UserCheckResponse {
            is_available: self.user_repo.find_by_email(email).await.is_none(),
        })
    }

    // pub async fn check_username(
    //     &self,
    //     id: Uuid,
    //     username: &str,
    // ) -> Result<UserCheckResponse, ApiError> {
    //     let is_available = if let Some(user) = self.user_repo.find_by_username(username).await {
    //         user.id == id
    //     } else {
    //         true
    //     };
    //     Ok(UserCheckResponse { is_available })
    // }

    pub fn verify_password(&self, user: &User, password: &str) -> bool {
        bcrypt::verify(password, user.password.clone().unwrap().as_str()).unwrap_or(false)
    }

    pub async fn create_user_with_email(
        &self,
        name: &str,
        institution: &str,
        email: &str,
        password: &str,
    ) -> Result<User, ApiError> {
        if self.user_repo.find_by_email(email).await.is_some() {
            return Err(UserError::UserAlreadyExists)?;
        }
        match self
            .user_repo
            .create_user_with_email(name, institution, email, password)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => Err(DbError::Str(e.to_string()))?,
        }
    }

    // pub async fn update_username(
    //     &self,
    //     id: Uuid,
    //     payload: UserUpdateUsernameRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     if let Some(user) = self.user_repo.find_by_username(&payload.user_name).await {
    //         if user.id == id {
    //             return Ok(UserUpdateResponse { state: true });
    //         } else {
    //             return Err(UserError::UsernameAlreadyExists)?;
    //         }
    //     }
    //     match self.user_repo.update_username(id, &payload.user_name).await {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_profile(
    //     &self,
    //     id: Uuid,
    //     payload: UserUpdateProfileRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .update_profile(
    //             id,
    //             payload.first_name,
    //             payload.middle_name,
    //             payload.second_name,
    //             payload.gender,
    //             payload.country,
    //             payload.city,
    //         )
    //         .await
    //     {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_avatar_url(
    //     &self,
    //     id: Uuid,
    //     avatar_url: Option<String>,
    // ) -> Result<UserUploadAvatarResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .update_avatar_url(id, avatar_url.clone())
    //         .await
    //     {
    //         Ok(result) => Ok(UserUploadAvatarResponse {
    //             state: result,
    //             avatar_url,
    //         }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_wallpaper_url(
    //     &self,
    //     id: Uuid,
    //     wallpaper_url: String,
    // ) -> Result<UserUploadWallPaperResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .update_wallpaper_url(id, wallpaper_url.to_owned())
    //         .await
    //     {
    //         Ok(result) => Ok(UserUploadWallPaperResponse {
    //             state: result,
    //             wallpaper_url,
    //         }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_setting_profile(
    //     &self,
    //     id: Uuid,
    //     payload: UserUpdateSettingProfileRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     let all_degrees = self.util_repo.get_degree("", None).await;
    //     let mut available_degree_ids = payload.degree.unwrap_or_default();
    //     for degree_name in payload.new_degree.unwrap_or_default() {
    //         let degree_exists = all_degrees
    //             .iter()
    //             .any(|d| d.degree_name.to_lowercase() == degree_name.to_lowercase());
    //         if !degree_exists {
    //             if let Ok(id) = self.util_repo.insert_degree(&degree_name).await {
    //                 available_degree_ids.push(id);
    //             }
    //         } else {
    //             if let Some(degree) = all_degrees
    //                 .iter()
    //                 .find(|d| d.degree_name.to_lowercase() == degree_name.to_lowercase())
    //             {
    //                 available_degree_ids.push(degree.id);
    //             }
    //         }
    //     }
    //     match self
    //         .user_repo
    //         .update_setting_profile(
    //             id,
    //             payload.first_name,
    //             payload.middle_name,
    //             payload.second_name,
    //             payload.gender,
    //             payload.country,
    //             payload.city,
    //             &available_degree_ids,
    //             payload.about_me,
    //             payload.expand_about_me,
    //         )
    //         .await
    //     {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn add_affiliation(
    //     &self,
    //     user: User,
    //     payload: UserAddAffiliationRequest,
    // ) -> Result<UserAddAffiliationResponse, ApiError> {
    //     let affiliations = self.user_repo.get_affiliations(user.id).await;
    //     let a = affiliations
    //         .iter()
    //         .filter(|a| {
    //             let title_match = a.title.as_ref().map(|t| t.trim().to_lowercase())
    //                 == payload.title.as_ref().map(|t| t.trim().to_lowercase());
    //             let institution_match = a.institution.as_ref().map(|i| i.trim().to_lowercase())
    //                 == payload
    //                     .institution
    //                     .as_ref()
    //                     .map(|i| i.trim().to_lowercase());
    //             title_match && institution_match
    //         })
    //         .count();
    //     if a > 0 {
    //         return Err(DbError::SomethingWentWrong(
    //             "This affiliation is a duplicate.".to_string(),
    //         ))?;
    //     }
    //     if let Ok(result) = self
    //         .user_repo
    //         .add_affiliation(
    //             user.id,
    //             payload.title,
    //             payload.institution,
    //             payload.department,
    //             payload.is_current,
    //             payload.institution_address,
    //             payload.line_2,
    //             payload.line_3,
    //             payload.country,
    //             payload.city,
    //             payload.postal_code,
    //             payload.work_phone_number,
    //         )
    //         .await
    //     {
    //         return Ok(UserAddAffiliationResponse {
    //             state: true,
    //             id: result,
    //         });
    //     } else {
    //         return Err(DbError::SomethingWentWrong(
    //             "Add affiliation failed".to_string(),
    //         ))?;
    //     }
    // }

    // pub async fn edit_affiliation(
    //     &self,
    //     payload: UserEditAffiliationRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .edit_affiliation(
    //             payload.id,
    //             payload.title,
    //             payload.institution,
    //             payload.department,
    //             payload.is_current,
    //             payload.institution_address,
    //             payload.line_2,
    //             payload.line_3,
    //             payload.country,
    //             payload.city,
    //             payload.postal_code,
    //             payload.work_phone_number,
    //         )
    //         .await
    //     {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn delete_affiliation(
    //     &self,
    //     payload: UserDeleteRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     if let Ok(result) = self.user_repo.delete_affiliation(payload.id).await {
    //         return Ok(UserUpdateResponse { state: result });
    //     } else {
    //         return Err(DbError::SomethingWentWrong(
    //             "Delete affiliation failed.".to_string(),
    //         ))?;
    //     }
    // }

    // pub async fn get_affiliations(&self, user_id: Uuid) -> Result<Vec<Affiliation>, ApiError> {
    //     Ok(self.user_repo.get_affiliations(user_id).await)
    // }

    // pub async fn edit_domain_expertise(
    //     &self,
    //     user_id: Uuid,
    //     payload: UserUpdateDomainExpertiseRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .update_domain_expertise(
    //             user_id,
    //             payload.expertise_domains,
    //             payload.years_of_experience,
    //             payload.years_of_experiences,
    //         )
    //         .await
    //     {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_nobleblocks_role(
    //     &self,
    //     user: User,
    //     payload: UserUpdateNobleblocksRoleRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .update_nobleblocks_role(
    //             user.id,
    //             payload.r_is_like,
    //             payload.r_expertise_domains,
    //             payload.r_number_review,
    //             payload.r_is_before_journals,
    //             payload.r_journals,
    //             payload.r_is_open,
    //             payload.r_number,
    //             payload.r_review_style,
    //             payload.e_is_like,
    //             payload.e_years,
    //             payload.e_is_before_journals,
    //             payload.e_journals,
    //             payload.e_is_open,
    //             payload.e_number,
    //             payload.e_decision_making,
    //             payload.c_is_like,
    //             payload.c_years,
    //             payload.c_article_types,
    //             payload.c_formatting_styles,
    //             payload.c_number,
    //         )
    //         .await
    //     {
    //         Ok(result) => {
    //             let result = if result {
    //                 self.user_repo
    //                     .update_user_role(
    //                         user.id,
    //                         true,
    //                         payload.r_is_like,
    //                         payload.e_is_like,
    //                         payload.c_is_like,
    //                         true,
    //                         payload.r_is_like && user.role_reviewer.unwrap_or_default(),
    //                         payload.e_is_like && user.role_editor.unwrap_or_default(),
    //                         payload.c_is_like && user.role_copy_editor.unwrap_or_default(),
    //                     )
    //                     .await
    //                     .unwrap_or_default()
    //             } else {
    //                 false
    //             };
    //             Ok(UserUpdateResponse { state: result })
    //         }
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn check_user_related(
    //     &self,
    //     id: Option<Uuid>,
    //     member_id: Uuid,
    //     types: u16,
    // ) -> Result<bool, ApiError> {
    //     if let Some(id) = id {
    //         Ok(self
    //             .user_repo
    //             .check_user_related(id, member_id, types)
    //             .await)
    //     } else {
    //         Ok(false)
    //     }
    // }

    // pub async fn update_roles(
    //     &self,
    //     id: Uuid,
    //     payload: UserUpdateRolesRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .update_roles(
    //             id,
    //             payload.role_editor,
    //             payload.role_reviewer,
    //             payload.role_copy_editor,
    //             payload.role_bounty_hunter,
    //         )
    //         .await
    //     {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_notification(
    //     &self,
    //     id: Uuid,
    //     payload: UserUpdateNotificationRequest,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self
    //         .user_repo
    //         .update_notification(id, payload.on_notification)
    //         .await
    //     {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_email(
    //     &self,
    //     id: Uuid,
    //     email: String,
    //     password: String,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self.user_repo.update_email(id, &email, &password).await {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_password(
    //     &self,
    //     id: Uuid,
    //     old_pwd: String,
    //     new_pwd: String,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     if let Some(user) = self.user_repo.find_by_user_id(id).await {
    //         if user.password.is_some()
    //             && !bcrypt::verify(old_pwd, user.password.unwrap().as_str()).unwrap_or(false)
    //         {
    //             return Err(ApiError::UserError(UserError::InvalidPassword));
    //         }
    //         match self.user_repo.update_password(id, &new_pwd).await {
    //             Ok(result) => Ok(UserUpdateResponse { state: result }),
    //             Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //         }
    //     } else {
    //         Err(ApiError::UserError(UserError::UserNotFound))
    //     }
    // }

    // pub async fn reset_password(
    //     &self,
    //     id: Uuid,
    //     new_pwd: String,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self.user_repo.update_password(id, &new_pwd).await {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_is_active(
    //     &self,
    //     id: Uuid,
    //     is_active: bool,
    // ) -> Result<UserUpdateResponse, ApiError> {
    //     match self.user_repo.update_is_active(id, is_active).await {
    //         Ok(result) => Ok(UserUpdateResponse { state: result }),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn get_members(
    //     &self,
    //     id: Option<Uuid>,
    //     me_id: Option<Uuid>,
    //     name: &str,
    //     user_type: u16,
    //     start: i32,
    //     limit: i32,
    // ) -> Result<Vec<UserInfo>, ApiError> {
    //     let degrees = self.util_repo.get_degree("", None).await;
    //     match self
    //         .user_repo
    //         .get_members(id, name, user_type, start, limit)
    //         .await
    //     {
    //         Ok(users) => {
    //             let mut user_infos = Vec::new();
    //             for user in users {
    //                 let is_following = self
    //                     .check_user_related(me_id, user.id, 0)
    //                     .await
    //                     .unwrap_or_default();
    //                 let is_follower = self
    //                     .check_user_related(me_id, user.id, 1)
    //                     .await
    //                     .unwrap_or_default();
    //                 let is_blocked = self
    //                     .check_user_related(me_id, user.id, 2)
    //                     .await
    //                     .unwrap_or_default();
    //                 let is_reported = self
    //                     .check_user_related(me_id, user.id, 3)
    //                     .await
    //                     .unwrap_or_default();
    //                 user_infos.push(user.to_user_info(
    //                     is_following,
    //                     is_follower,
    //                     is_blocked,
    //                     is_reported,
    //                     &degrees,
    //                 ));
    //             }
    //             Ok(user_infos)
    //         }
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn follow_user(
    //     &self,
    //     follower_id: Uuid,
    //     followed_id: Uuid,
    // ) -> Result<bool, ApiError> {
    //     match self.user_repo.follow_user(follower_id, followed_id).await {
    //         Ok(result) => Ok(result),
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn report_user(
    //     &self,
    //     user_id: Uuid,
    //     reported_id: Uuid,
    //     description: String,
    //     status: Option<bool>,
    // ) -> Result<bool, ApiError> {
    //     match self
    //         .user_repo
    //         .report_user(user_id, reported_id, description, status.unwrap_or(true))
    //         .await
    //     {
    //         Ok(result) => Ok(result),
    //         Err(_) => Err(DbError::SomethingWentWrong(
    //             "Report user failed".to_string(),
    //         ))?,
    //     }
    // }

    // pub async fn block_user(
    //     &self,
    //     user_id: Uuid,
    //     blocked_id: Uuid,
    //     reason: Option<String>,
    // ) -> Result<bool, ApiError> {
    //     match self.user_repo.block_user(user_id, blocked_id, reason).await {
    //         Ok(result) => Ok(result),
    //         Err(_) => Err(DbError::SomethingWentWrong("Block user failed".to_string()))?,
    //     }
    // }

    // pub async fn get_suggested_user(
    //     &self,
    //     id: Uuid,
    //     count: i32,
    // ) -> Result<Vec<UserInfo>, ApiError> {
    //     match self.user_repo.get_suggested_user(id, count).await {
    //         Ok(users) => {
    //             let degrees = self.util_repo.get_degree("", None).await;
    //             let mut user_infos = Vec::new();
    //             for user in users {
    //                 let is_following = self.user_repo.check_user_related(id, user.id, 0).await;
    //                 let is_follower = self.user_repo.check_user_related(id, user.id, 1).await;
    //                 let is_blocked = self.user_repo.check_user_related(id, user.id, 2).await;
    //                 let is_reported = self.user_repo.check_user_related(id, user.id, 3).await;
    //                 user_infos.push(user.to_user_info(
    //                     is_following,
    //                     is_follower,
    //                     is_blocked,
    //                     is_reported,
    //                     &degrees,
    //                 ));
    //             }
    //             Ok(user_infos)
    //         }
    //         Err(e) => Err(DbError::SomethingWentWrong(e.to_string()))?,
    //     }
    // }

    // pub async fn update_rating(
    //     &self,
    //     id: Uuid,
    //     rating: i32,
    //     is_increase: bool,
    // ) -> Result<bool, ApiError> {
    //     Ok(self
    //         .user_repo
    //         .update_rating(id, rating, is_increase)
    //         .await
    //         .unwrap_or_default())
    // }
}
