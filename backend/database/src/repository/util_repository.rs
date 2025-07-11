use crate::pool::DatabasePool;
use sqlx::{self};
use std::sync::Arc;
use types::models::{Category, City, Country, Value};
use uuid::Uuid;

#[derive(Clone)]
pub struct UtilRepository {
    pub(crate) db_conn: Arc<DatabasePool>,
}

impl UtilRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn get_country(&self, country: &str, limit: u32) -> Vec<Country> {
        sqlx::query_as::<_, Country>("SELECT DISTINCT country, iso2 FROM city_list WHERE LOWER(country) like LOWER($1) ORDER BY country LIMIT $2")
            .bind(format!("{country}%"))
            .bind(limit as i32)
            .fetch_all(self.db_conn.get_pool())
            .await
            .unwrap_or_default()
    }

    pub async fn get_city(&self, country: &str, city: &str) -> Vec<City> {
        sqlx::query_as::<_, City>(
            "SELECT DISTINCT city_ascii AS city FROM city_list WHERE country = $1 and LOWER(city_ascii) like LOWER($2) ORDER BY city_ascii",
        )
        .bind(country)
        .bind(format!("{city}%"))
        .fetch_all(self.db_conn.get_pool())
        .await
        .unwrap_or_default()
    }

    // pub async fn get_degree(&self, degree: &str, is_available: Option<i16>) -> Vec<Degree> {
    //     let query = if let Some(available) = is_available {
    //         format!("SELECT * FROM degree WHERE degree_name like $1 AND is_available = {} ORDER BY degree_name", available)
    //     } else {
    //         format!("SELECT * FROM degree WHERE degree_name like $1 ORDER BY degree_name")
    //     };
    //     sqlx::query_as::<_, Degree>(&query)
    //         .bind(format!("%{degree}%"))
    //         .fetch_all(self.db_conn.get_pool())
    //         .await
    //         .unwrap_or_default()
    // }

    // pub async fn get_degrees_by_ids(&self, ids: &Vec<Uuid>) -> Vec<Degree> {
    //     sqlx::query_as::<_, Degree>("SELECT * FROM degree WHERE id = ANY($1)")
    //         .bind(ids)
    //         .fetch_all(self.db_conn.get_pool())
    //         .await
    //         .unwrap_or_default()
    // }

    // pub async fn insert_degree(&self, degree: &str) -> Result<Uuid, SqlxError> {
    //     let row: InsertResult =
    //         sqlx::query_as("INSERT INTO degree (degree_name) VALUES($1) RETURNING id")
    //             .bind(degree)
    //             .fetch_one(self.db_conn.get_pool())
    //             .await?;
    //     Ok(row.id)
    // }

    // pub async fn update_degree(
    //     &self,
    //     id: Uuid,
    //     degree_name: &str,
    //     abbreviation: &str,
    //     is_available: i16,
    // ) -> bool {
    //     sqlx::query("UPDATE degree SET degree_name = $1, abbreviation = $2, is_available = $3 WHERE id = $4")
    //         .bind(degree_name)
    //         .bind(abbreviation)
    //         .bind(is_available)
    //         .bind(id)
    //         .execute(self.db_conn.get_pool())
    //         .await.unwrap_or_default().rows_affected() == 1
    // }

    // pub async fn get_hashtags(
    //     &self,
    //     hashtag: &str,
    //     limit: i32,
    //     order_by: i32,
    // ) -> Vec<HashTagsInfo> {
    //     sqlx::query_as::<_, HashTagsInfo>(&format!(
    //         "SELECT * FROM hashtags WHERE hashtag_name ILIKE $1 ORDER BY {} LIMIT $2",
    //         if order_by == 0 {
    //             "hashtag_name ASC"
    //         } else {
    //             "usage_count DESC"
    //         }
    //     ))
    //     .bind(format!("%{hashtag}%"))
    //     .bind(limit)
    //     .fetch_all(self.db_conn.get_pool())
    //     .await
    //     .unwrap_or_default()
    // }

    // pub async fn insert_hashtag(&self, hashtag: &str) -> Option<HashTagsInfo> {
    //     sqlx::query_as::<_, HashTagsInfo>(
    //         "INSERT INTO hashtags(hashtag_name, is_available, usage_count) VALUES($1, $2, $3) RETURNING *",
    //     )
    //     .bind(hashtag)
    //     .bind(true)
    //     .bind(1)
    //     .fetch_optional(self.db_conn.get_pool())
    //     .await
    //     .unwrap_or_default()
    // }

    // pub async fn update_hashtag(&self, hashtag: &str) -> Option<HashTagsInfo> {
    //     sqlx::query_as::<_, HashTagsInfo>(
    //         "UPDATE hashtags SET usage_count = usage_count + 1 WHERE LOWER(hashtag_name) = LOWER($1) RETURNING *",
    //     )
    //     .bind(hashtag)
    //     .fetch_optional(self.db_conn.get_pool())
    //     .await
    //     .unwrap_or_default()
    // }

    // pub async fn insert_post_hashtag(&self, post_id: Uuid, hashtag_id: Uuid) -> bool {
    //     sqlx::query("INSERT INTO post_hashtags (post_id, hashtag_id) VALUES ($1, $2)")
    //         .bind(post_id)
    //         .bind(hashtag_id)
    //         .execute(self.db_conn.get_pool())
    //         .await
    //         .unwrap_or_default()
    //         .rows_affected()
    //         == 1
    // }

    pub async fn get_categories(
        &self,
        name: &str,
        is_available: Option<bool>,
        start: i32,
        limit: i32,
    ) -> Vec<Category> {
        let query = if let Some(available) = is_available {
            format!("SELECT * FROM category WHERE name ILIKE $1 AND is_available = {} ORDER BY CASE WHEN name ILIKE $4 THEN 0 ELSE 1 END, name LIMIT $2 OFFSET $3", available)
        } else {
            format!("SELECT * FROM category WHERE name ILIKE $1 ORDER BY CASE WHEN name ILIKE $4 THEN 0 ELSE 1 END, name LIMIT $2 OFFSET $3")
        };
        sqlx::query_as::<_, Category>(&query)
            .bind(format!("%{name}%"))
            .bind(limit)
            .bind(start)
            .bind(format!("{name}%"))
            .fetch_all(self.db_conn.get_pool())
            .await
            .unwrap_or_default()
    }

    pub async fn get_category_by_id(&self, id: Uuid) -> Option<Category> {
        sqlx::query_as::<_, Category>("SELECT * FROM category WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or_default()
    }

    pub async fn get_category_by_name(&self, category: &str) -> Option<Category> {
        sqlx::query_as::<_, Category>("SELECT * FROM category WHERE name = $1")
            .bind(category)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or_default()
    }

    pub async fn insert_category(&self, category: &str) -> Option<Category> {
        sqlx::query_as::<_, Category>("INSERT INTO category(name) VALUES($1) RETURNING *")
            .bind(category)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or_default()
    }

    pub async fn get_category_by_ids(&self, ids: &Vec<Uuid>) -> Vec<Category> {
        sqlx::query_as::<_, Category>(
            "SELECT * FROM category WHERE id = ANY($1) AND is_available = true",
        )
        .bind(ids)
        .fetch_all(self.db_conn.get_pool())
        .await
        .unwrap_or_default()
    }

    pub async fn update_category(&self, id: Uuid, name: &str, is_available: bool) -> bool {
        sqlx::query("UPDATE category SET name = $1, is_available = $2 WHERE id = $3")
            .bind(name)
            .bind(is_available)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await
            .unwrap_or_default()
            .rows_affected()
            == 1
    }

    pub async fn delete_category(&self, id: Uuid) -> bool {
        sqlx::query("DELETE FROM category WHERE id = $1")
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await
            .unwrap_or_default()
            .rows_affected()
            == 1
    }

    pub async fn get_last_block_number(&self) -> Result<Option<String>, sqlx::Error> {
        let value = sqlx::query_as!(
            Value,
            "SELECT * from values WHERE key = 'last_block_number'",
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;
        Ok(value.map(|v| v.value))
    }

    pub async fn upsert_last_block_number(
        &self,
        last_block_number: &str,
    ) -> Result<String, sqlx::Error> {
        let value = sqlx::query_as!(
            Value,
            r#"
                INSERT INTO values (key, value)
                VALUES ('last_block_number', $1)
                ON CONFLICT (key)
                DO UPDATE SET value = EXCLUDED.value
                RETURNING *
            "#,
            last_block_number,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;
        Ok(value.value)
    }
}
