use crate::pool::DatabasePool;
use chrono::Utc;
use sqlx::{self, Error as SqlxError};
use std::sync::Arc;
use types::{models::User, UserRoleType, UserTierType};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl UserRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    // pub async fn tempuser_by_email(&self, email: &str) -> Option<TempUser> {
    //     sqlx::query_as::<_, TempUser>("SELECT * FROM temp_user WHERE email = $1")
    //         .bind(email)
    //         .fetch_optional(self.db_conn.get_pool())
    //         .await
    //         .unwrap_or(None)
    // }

    // pub async fn create_tempuser_with_email(
    //     &self,
    //     email: &str,
    //     password: &str,
    //     verify_type: &str,
    //     passkey: &str,
    //     try_limit: i16,
    //     iat: i64,
    //     exp: i64,
    //     now: DateTime<Utc>,
    // ) -> Result<bool, SqlxError> {
    //     let insert = sqlx::query_as!(
    //         TempUser,
    //         "INSERT INTO temp_user (email, password, verify_type, passkey, try_limit, iat, exp, created_at, updated_at)
    //         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    //         email,
    //         password,
    //         verify_type,
    //         passkey,
    //         try_limit,
    //         iat,
    //         exp,
    //         now,
    //         now,
    //     )
    //     .execute(self.db_conn.get_pool())
    //     .await?;

    //     return Ok(insert.rows_affected() == 1);
    // }

    // pub async fn update_tempuser_with_email(
    //     &self,
    //     email: &str,
    //     password: &str,
    //     verify_type: &str,
    //     passkey: &str,
    //     try_limit: i16,
    //     iat: i64,
    //     exp: i64,
    //     now: DateTime<Utc>,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query_as!(
    //         TempUser,
    //         "UPDATE temp_user SET password = $1, verify_type = $2, passkey = $3, try_limit = $4, iat = $5, exp = $6, updated_at = $7 WHERE email = $8",
    //         password,
    //         verify_type,
    //         passkey,
    //         try_limit,
    //         iat,
    //         exp,
    //         now,
    //         email,
    //     )
    //     .execute(self.db_conn.get_pool())
    //     .await?;

    //     return Ok(update.rows_affected() >= 1);
    // }

    pub async fn get_user_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_user_by_gmail(&self, gmail: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE gmail = $1")
            .bind(gmail)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_website(&self, web_site: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE web_site = $1")
            .bind(web_site)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_linkedin(&self, linkedin: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE linkedin = $1")
            .bind(linkedin)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_orc_id(&self, orc_id: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE orc_id = $1")
            .bind(orc_id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    pub async fn create_user_with_email(
        &self,
        name: &str,
        institution: &str,
        email: &str,
        password: &str,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, password, institution, tier, verified_email)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(name)
        .bind(email)
        .bind(password)
        .bind(institution)
        .bind(UserTierType::Bronze.to_string())
        .bind(true)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        return Ok(user);
    }

    pub async fn create_user_with_google(&self, gmail: &str) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, verified_email, gmail)
            VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(gmail)
        .bind(true)
        .bind(gmail)
        .fetch_one(self.db_conn.get_pool())
        .await?;
        return Ok(user);
    }

    pub async fn update_gmail(&self, id: Uuid, gmail: Option<String>) -> Result<bool, SqlxError> {
        let row = sqlx::query("UPDATE users SET gmail = $1 WHERE id = $2")
            .bind(gmail)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(row.rows_affected() == 1)
    }

    pub async fn get_editors(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<User>, SqlxError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE $1 = ANY(roles) LIMIT $2 OFFSET $3",
        )
        .bind(UserRoleType::Editor.to_string())
        .bind(limit.unwrap_or(10))
        .bind(offset.unwrap_or(0))
        .fetch_all(self.db_conn.get_pool())
        .await?;
        Ok(users)
    }

    pub async fn update_user_onboarding(
        &self,
        id: Uuid,
        name: &str,
        institution: &str,
        bio: &str,
        roles: Vec<String>,
        interests: Vec<String>,
        wallet_address: &str,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>("UPDATE users SET name = $1, institution = $2, bio = $3, roles = $4, interests = $5, wallet_address = $6, updated_at = $7 WHERE id = $8 RETURNING *")
            .bind(name)
            .bind(institution)
            .bind(bio)
            .bind(roles)
            .bind(interests)
            .bind(wallet_address)
            .bind(Utc::now())
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(user)
    }

    // pub async fn update_website(
    //     &self,
    //     id: Uuid,
    //     web_site: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE users SET web_site = $1 WHERE id = $2")
    //         .bind(web_site)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_linkedin(
    //     &self,
    //     id: Uuid,
    //     linkedin: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE users SET linkedin = $1 WHERE id = $2")
    //         .bind(linkedin)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_orc_id(&self, id: Uuid, orc_id: Option<String>) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE users SET orc_id = $1 WHERE id = $2")
    //         .bind(orc_id)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_google_scholar(
    //     &self,
    //     id: Uuid,
    //     google_scholar: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE users SET google_scholar = $1 WHERE id = $2")
    //         .bind(google_scholar)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_principal(
    //     &self,
    //     id: Uuid,
    //     principal: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row = sqlx::query("UPDATE users SET principal = $1 WHERE id = $2")
    //         .bind(principal)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn create_user_with_principal(
    //     &self,
    //     principal: &str,
    //     noble_id: &str,
    // ) -> Result<User, SqlxError> {
    //     let now = Utc::now();
    //     let user = sqlx::query_as::<_, User>(
    //         "INSERT INTO users (noble_id, principal, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *"
    //     )
    //     .bind(noble_id)
    //     .bind(principal)
    //     .bind(now)
    //     .bind(now)
    //     .fetch_one(self.db_conn.get_pool())
    //     .await?;

    //     return Ok(user);
    // }

    // pub async fn update_twitter(
    //     &self,
    //     id: Uuid,
    //     twitter_id: Option<String>,
    //     twitter_username: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row =
    //         sqlx::query("UPDATE users SET twitter_id = $1, twitter_username = $2 WHERE id = $3")
    //             .bind(twitter_id)
    //             .bind(twitter_username)
    //             .bind(id)
    //             .execute(self.db_conn.get_pool())
    //             .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn create_user_with_twitter(
    //     &self,
    //     twitter_id: &str,
    //     twitter_username: &str,
    //     noble_id: &str,
    // ) -> Result<bool, SqlxError> {
    //     let now = Utc::now();
    //     let insert = sqlx::query_as!(
    //         User,
    //         "INSERT INTO users (twitter_id, twitter_username, noble_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
    //         twitter_id,
    //         twitter_username,
    //         noble_id,
    //         now,
    //         now,
    //     )
    //     .execute(self.db_conn.get_pool())
    //     .await?;

    //     return Ok(insert.rows_affected() == 1);
    // }

    // pub async fn update_telegram(
    //     &self,
    //     id: Uuid,
    //     telegram_id: Option<i64>,
    //     telegram_username: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let row =
    //         sqlx::query("UPDATE users SET telegram_id = $1, telegram_username = $2 WHERE id = $3")
    //             .bind(telegram_id)
    //             .bind(telegram_username)
    //             .bind(id)
    //             .execute(self.db_conn.get_pool())
    //             .await?;
    //     Ok(row.rows_affected() == 1)
    // }

    // pub async fn update_profile(
    //     &self,
    //     id: Uuid,
    //     first_name: Option<String>,
    //     middle_name: Option<String>,
    //     second_name: Option<String>,
    //     gender: Option<i16>,
    //     country: Option<String>,
    //     city: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query_as!(
    //         User,
    //         "UPDATE users SET first_name = $1, middle_name = $2, second_name = $3, gender = $4, country = $5, city = $6, updated_at = $7, onboarding_step = 2 WHERE id = $8",
    //         first_name,
    //         middle_name,
    //         second_name,
    //         gender,
    //         country,
    //         city,
    //         Utc::now(),
    //         id
    //     )
    //     .execute(self.db_conn.get_pool())
    //     .await?;

    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_avatar_url(
    //     &self,
    //     id: Uuid,
    //     avatar_url: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query_as!(
    //         User,
    //         "UPDATE users SET avatar_url = $1, updated_at = $2 WHERE id = $3",
    //         avatar_url,
    //         Utc::now(),
    //         id
    //     )
    //     .execute(self.db_conn.get_pool())
    //     .await?;

    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_wallpaper_url(
    //     &self,
    //     id: Uuid,
    //     wallpaper_url: String,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query_as!(
    //         User,
    //         "UPDATE users SET wallpaper_url = $1, updated_at = $2 WHERE id = $3",
    //         wallpaper_url,
    //         Utc::now(),
    //         id
    //     )
    //     .execute(self.db_conn.get_pool())
    //     .await?;

    //     return Ok(update.rows_affected() >= 1);
    // }

    pub async fn update_username(&self, id: Uuid, username: &str) -> Result<bool, SqlxError> {
        let update = sqlx::query("UPDATE users SET username = $1, updated_at = $2 WHERE id = $3")
            .bind(username)
            .bind(Utc::now())
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await?;
        return Ok(update.rows_affected() >= 1);
    }

    // pub async fn update_setting_profile(
    //     &self,
    //     id: Uuid,
    //     first_name: Option<String>,
    //     middle_name: Option<String>,
    //     second_name: Option<String>,
    //     gender: Option<i16>,
    //     country: Option<String>,
    //     city: Option<String>,
    //     degree: &Vec<Uuid>,
    //     about_me: Option<String>,
    //     expand_about_me: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query("UPDATE users SET first_name = $1, middle_name = $2, second_name = $3, gender = $4, country = $5, city = $6, degree = $7, about_me = $8, expand_about_me = $9, updated_at = $10 WHERE id = $11")
    //         .bind(first_name)
    //         .bind(middle_name)
    //         .bind(second_name)
    //         .bind(gender)
    //         .bind(country)
    //         .bind(city)
    //         .bind(degree)
    //         .bind(about_me)
    //         .bind(expand_about_me)
    //         .bind(Utc::now())
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_user_role(
    //     &self,
    //     id: Uuid,
    //     role_author_unverified: bool,
    //     role_reviewer_unverified: bool,
    //     role_editor_unverified: bool,
    //     role_copy_editor_unverified: bool,
    //     role_author: bool,
    //     role_reviewer: bool,
    //     role_editor: bool,
    //     role_copy_editor: bool,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query("UPDATE users SET role_author_unverified = $1, role_reviewer_unverified = $2, role_editor_unverified = $3, role_author = $4, role_reviewer = $5, role_editor = $6, role_copy_editor_unverified = $7, role_copy_editor = $8, updated_at = $9 WHERE id = $10")
    //         .bind(role_author_unverified)
    //         .bind(role_reviewer_unverified)
    //         .bind(role_editor_unverified)
    //         .bind(role_author)
    //         .bind(role_reviewer)
    //         .bind(role_editor)
    //         .bind(role_copy_editor_unverified)
    //         .bind(role_copy_editor)
    //         .bind(Utc::now())
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     return Ok(update.rows_affected() == 1);
    // }

    // pub async fn get_affiliations(&self, user_id: Uuid) -> Vec<Affiliation> {
    //     sqlx::query_as::<_, Affiliation>("SELECT * FROM affiliation WHERE user_id = $1")
    //         .bind(user_id)
    //         .fetch_all(self.db_conn.get_pool())
    //         .await
    //         .unwrap_or_default()
    // }

    // pub async fn add_affiliation(
    //     &self,
    //     id: Uuid,
    //     title: Option<String>,
    //     institution: Option<String>,
    //     department: Option<String>,
    //     is_current: Option<bool>,
    //     institution_address: Option<String>,
    //     line_2: Option<String>,
    //     line_3: Option<String>,
    //     country: Option<String>,
    //     city: Option<String>,
    //     postal_code: Option<String>,
    //     work_phone_number: Option<String>,
    // ) -> Result<Uuid, SqlxError> {
    //     let row: InsertResult = sqlx::query_as("INSERT INTO affiliation(user_id, title, institution, department, is_current, institution_address, line_2, line_3, country, city, postal_code, work_phone_number, created_at, updated_at) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) RETURNING id")
    //         .bind(id)
    //         .bind(title)
    //         .bind(institution)
    //         .bind(department)
    //         .bind(is_current)
    //         .bind(institution_address)
    //         .bind(line_2)
    //         .bind(line_3)
    //         .bind(country)
    //         .bind(city)
    //         .bind(postal_code)
    //         .bind(work_phone_number)
    //         .bind(Utc::now())
    //         .bind(Utc::now())
    //         .fetch_one(self.db_conn.get_pool())
    //         .await?;
    //     let inserted_id: Uuid = row.id;
    //     return Ok(inserted_id);
    // }

    // pub async fn edit_affiliation(
    //     &self,
    //     id: Uuid,
    //     title: Option<String>,
    //     institution: Option<String>,
    //     department: Option<String>,
    //     is_current: Option<bool>,
    //     institution_address: Option<String>,
    //     line_2: Option<String>,
    //     line_3: Option<String>,
    //     country: Option<String>,
    //     city: Option<String>,
    //     postal_code: Option<String>,
    //     work_phone_number: Option<String>,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query("UPDATE affiliation SET title = $1, institution = $2, department = $3, is_current = $4, institution_address = $5, line_2 = $6, line_3 = $7, country = $8, city = $9, postal_code = $10, work_phone_number = $11, updated_at = $12 WHERE id = $13")
    //         .bind(title)
    //         .bind(institution)
    //         .bind(department)
    //         .bind(is_current)
    //         .bind(institution_address)
    //         .bind(line_2)
    //         .bind(line_3)
    //         .bind(country)
    //         .bind(city)
    //         .bind(postal_code)
    //         .bind(work_phone_number)
    //         .bind(Utc::now())
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn delete_affiliation(&self, id: Uuid) -> Result<bool, SqlxError> {
    //     let update = sqlx::query("DELETE FROM affiliation WHERE id = $1")
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_domain_expertise(
    //     &self,
    //     id: Uuid,
    //     expertise_domains: Option<Vec<String>>,
    //     years_of_experience: Option<i16>,
    //     years_of_experiences: Option<Vec<i16>>,
    // ) -> Result<bool, SqlxError> {
    //     let res = sqlx::query(
    //         "UPDATE users SET expertise_domains = $1, years_of_experience = $2, years_of_experiences = $3 WHERE id = $4",
    //     )
    //     .bind(expertise_domains)
    //     .bind(years_of_experience)
    //     .bind(years_of_experiences)
    //     .bind(id)
    //     .execute(self.db_conn.get_pool())
    //     .await?;
    //     Ok(res.rows_affected() == 1)
    // }

    // pub async fn update_nobleblocks_role(
    //     &self,
    //     id: Uuid,
    //     r_is_like: bool,
    //     r_expertise_domains: Option<Vec<String>>,
    //     r_number_review: Option<i16>,
    //     r_is_before_journals: Option<bool>,
    //     r_journals: Option<Vec<String>>,
    //     r_is_open: Option<bool>,
    //     r_number: Option<i16>,
    //     r_review_style: Option<Vec<i16>>,
    //     e_is_like: bool,
    //     e_years: Option<i16>,
    //     e_is_before_journals: Option<bool>,
    //     e_journals: Option<Vec<String>>,
    //     e_is_open: Option<bool>,
    //     e_number: Option<i16>,
    //     e_decision_making: Option<bool>,
    //     c_is_like: bool,
    //     c_years: Option<i16>,
    //     c_article_types: Option<Vec<Uuid>>,
    //     c_formatting_styles: Option<Vec<String>>,
    //     c_number: Option<i16>,
    // ) -> Result<bool, SqlxError> {
    //     let res = sqlx::query("UPDATE users SET r_is_like = $1, e_is_like = $2, c_is_like = $3, r_expertise_domains = $4, r_number_review = $5, r_is_before_journals = $6, r_journals = $7, r_is_open = $8, r_number = $9, r_review_style = $10, e_years = $11, e_is_before_journals = $12, e_journals = $13, e_is_open = $14, e_number = $15, e_decision_making = $16, c_years = $17, c_article_types = $18, c_formatting_styles = $19, c_number = $20 WHERE id = $21")
    //         .bind(r_is_like)
    //         .bind(e_is_like)
    //         .bind(c_is_like)
    //         .bind(if r_is_like {r_expertise_domains} else {None})
    //         .bind(if r_is_like {r_number_review} else {None})
    //         .bind(if r_is_like {r_is_before_journals} else {None})
    //         .bind(if r_is_like {r_journals} else {None})
    //         .bind(if r_is_like {r_is_open} else {None})
    //         .bind(if r_is_like {r_number} else {None})
    //         .bind(if r_is_like {r_review_style} else {None})
    //         .bind(if e_is_like {e_years.or(Some(0))} else {None})
    //         .bind(if e_is_like {e_is_before_journals} else {None})
    //         .bind(if e_is_like {e_journals} else {None})
    //         .bind(if e_is_like {e_is_open} else {None})
    //         .bind(if e_is_like {e_number} else {None})
    //         .bind(if e_is_like {e_decision_making} else {None})
    //         .bind(if c_is_like {c_years.or(Some(0))} else {None})
    //         .bind(if c_is_like {c_article_types} else {None})
    //         .bind(if c_is_like {c_formatting_styles} else {None})
    //         .bind(if c_is_like {c_number} else {None})
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(res.rows_affected() == 1)
    // }

    // pub async fn update_roles(
    //     &self,
    //     id: Uuid,
    //     editor: Option<bool>,
    //     reviewer: Option<bool>,
    //     copy_editor: Option<bool>,
    //     bounty_hunter: Option<bool>,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query("UPDATE users SET role_editor = $1, role_reviewer = $2, role_copy_editor = $3, role_bounty_hunter = $4, updated_at = $5 WHERE id = $6")
    //         .bind(editor)
    //         .bind(reviewer)
    //         .bind(copy_editor)
    //         .bind(bounty_hunter)
    //         .bind(Utc::now())
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_publishing(
    //     &self,
    //     id: Uuid,
    //     allowed_comments: bool,
    // ) -> Result<bool, SqlxError> {
    //     let update =
    //         sqlx::query("UPDATE users SET allowed_comments = $1, updated_at = $2 WHERE id = $3")
    //             .bind(allowed_comments)
    //             .bind(Utc::now())
    //             .bind(id)
    //             .execute(self.db_conn.get_pool())
    //             .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_notification(
    //     &self,
    //     id: Uuid,
    //     on_notification: bool,
    // ) -> Result<bool, SqlxError> {
    //     let update =
    //         sqlx::query("UPDATE users SET on_notification = $1, updated_at = $2 WHERE id = $3")
    //             .bind(on_notification)
    //             .bind(Utc::now())
    //             .bind(id)
    //             .execute(self.db_conn.get_pool())
    //             .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_email(
    //     &self,
    //     id: Uuid,
    //     email: &str,
    //     password: &str,
    // ) -> Result<bool, SqlxError> {
    //     let update = sqlx::query("UPDATE users SET email = $1, verified_email = $2, email_updated_at = $3, password = $4, password_updated_at = $5, updated_at = $6 WHERE id = $7")
    //         .bind(email)
    //         .bind(true)
    //         .bind(Utc::now())
    //         .bind(password)
    //         .bind(Utc::now())
    //         .bind(Utc::now())
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn update_password(&self, id: Uuid, password: &str) -> Result<bool, SqlxError> {
    //     let update = sqlx::query("UPDATE users SET password = $1, password_updated_at = $2, updated_at = $3 WHERE id = $4")
    //         .bind(bcrypt::hash(password, 12).unwrap())
    //         .bind(Utc::now())
    //         .bind(Utc::now())
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     return Ok(update.rows_affected() >= 1);
    // }

    // pub async fn get_user_info_by_id(&self, user_id: Uuid) -> Option<UserInfo> {
    //     if let Some(user) = self.find_by_user_id(user_id).await {
    //         return Some(user.to_user_info());
    //     }
    //     None
    // }

    // pub async fn delete_user(&self, user_id: Uuid) -> bool {
    //     let row = sqlx::query("DELETE FROM users WHERE id = $1")
    //         .bind(user_id)
    //         .execute(self.db_conn.get_pool())
    //         .await
    //         .unwrap_or_default();
    //     let result = row.rows_affected() == 1;
    //     if result { // will delete all data of this user
    //     }
    //     result
    // }

    // pub async fn get_users_by_ids(&self, user_ids: Vec<Uuid>) -> Result<Vec<User>, SqlxError> {
    //     let users = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ANY($1)")
    //         .bind(user_ids)
    //         .fetch_all(self.db_conn.get_pool())
    //         .await
    //         .unwrap_or_default();
    //     Ok(users)
    // }

    // pub async fn update_rating(
    //     &self,
    //     id: Uuid,
    //     rating: i32,
    //     is_increase: bool,
    // ) -> Result<bool, SqlxError> {
    //     let query = format!(
    //         "UPDATE users SET rating = rating {} $1 WHERE id = $2",
    //         if is_increase { "+" } else { "-" }
    //     );
    //     let row = sqlx::query(&query)
    //         .bind(rating)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await?;
    //     Ok(row.rows_affected() == 1)
    // }
}
