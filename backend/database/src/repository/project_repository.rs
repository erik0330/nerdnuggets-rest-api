// use crate::pool::DatabasePool;
// use chrono::{DateTime, Utc};
// use sqlx::{self, Error as SqlxError};
// use std::sync::Arc;
// use types::{
//     models::{Affiliation, User, UserInfo},
//     InsertResult, UserTierType,
// };
// use uuid::Uuid;

// #[derive(Clone)]
// pub struct ProjectRepository {
//     pub(crate) db_conn: Arc<DatabasePool>,
// }

// impl ProjectRepository {
//     pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
//         Self {
//             db_conn: Arc::clone(db_conn),
//         }
//     }

//     pub async fn get_project_by_id(&self, id: Uuid) -> Option<User> {
//         sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
//             .bind(id)
//             .fetch_optional(self.db_conn.get_pool())
//             .await
//             .unwrap_or(None)
//     }

//     pub async fn find_by_user_name(&self, user_name: &str, increase: bool) -> Option<User> {
//         if increase {
//             sqlx::query("UPDATE users SET count_view = count_view + 1 WHERE user_name = $1")
//                 .bind(user_name)
//                 .execute(self.db_conn.get_pool())
//                 .await
//                 .unwrap_or_default();
//         }
//         sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_name = $1")
//             .bind(user_name)
//             .fetch_optional(self.db_conn.get_pool())
//             .await
//             .unwrap_or(None)
//     }

//     pub async fn create_user_with_email(
//         &self,
//         username: &str,
//         institution: &str,
//         email: &str,
//         password: &str,
//     ) -> Result<User, SqlxError> {
//         let user = sqlx::query_as::<_, User>(
//             "INSERT INTO users (username, email, password, institution, tier)
//             VALUES ($1, $2, $3, $4, $5) RETURNING *",
//         )
//         .bind(username)
//         .bind(email)
//         .bind(password)
//         .bind(institution)
//         .bind(UserTierType::Bronze.to_string())
//         .fetch_one(self.db_conn.get_pool())
//         .await?;
//         return Ok(user);
//     }

//     pub async fn create_user_with_google(&self, gmail: &str) -> Result<User, SqlxError> {
//         let user = sqlx::query_as::<_, User>(
//             "INSERT INTO users (email, verified_email, gmail)
//             VALUES ($1, $2, $3) RETURNING *",
//         )
//         .bind(gmail)
//         .bind(true)
//         .bind(gmail)
//         .fetch_one(self.db_conn.get_pool())
//         .await?;
//         return Ok(user);
//     }

//     pub async fn update_gmail(&self, id: Uuid, gmail: Option<String>) -> Result<bool, SqlxError> {
//         let row = sqlx::query("UPDATE users SET gmail = $1 WHERE id = $2")
//             .bind(gmail)
//             .bind(id)
//             .execute(self.db_conn.get_pool())
//             .await?;
//         Ok(row.rows_affected() == 1)
//     }
// }
