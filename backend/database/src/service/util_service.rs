use crate::{pool::DatabasePool, repository::UtilRepository};
use serde_json::Value;
use std::{str::FromStr, sync::Arc};
use types::{
    dto::{GetCityResponse, GetCountryResponse, GetInstitutionDetailItem, GetInstitutionsItem},
    error::{ApiError, DbError},
    models::Category,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UtilService {
    util_repo: UtilRepository,
}

impl UtilService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            util_repo: UtilRepository::new(db_conn),
        }
    }

    pub async fn get_country(
        &self,
        country: String,
        limit: u32,
    ) -> Result<GetCountryResponse, ApiError> {
        Ok(GetCountryResponse {
            country_list: self.util_repo.get_country(&country, limit).await,
        })
    }

    pub async fn get_city(
        &self,
        country: String,
        city: String,
    ) -> Result<GetCityResponse, ApiError> {
        Ok(GetCityResponse {
            city_list: self.util_repo.get_city(&country, &city).await,
        })
    }

    pub async fn get_institutions(
        &self,
        key: String,
        search_type: String,
        query: String,
    ) -> Result<Vec<GetInstitutionsItem>, ApiError> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/place/textsearch/json?type={}&key={}&query={}",
            search_type, key, query
        );
        if let Ok(response) = reqwest::get(url).await {
            let json: Value = response.json().await.unwrap_or_default();
            let max_result = 10;
            // let mut unique_names: HashSet<GetInstitutionsItem> = HashSet::new();
            let mut institutions = Vec::new();
            json["results"]
                .as_array()
                .unwrap_or(&Vec::new())
                .into_iter()
                .take(max_result)
                .for_each(|result| {
                    if let Some(name) = result["name"].as_str() {
                        institutions.push(GetInstitutionsItem {
                            name: name.to_string(),
                            formatted_address: result["formatted_address"]
                                .as_str()
                                .unwrap_or_default()
                                .to_string(),
                            place_id: result["place_id"].as_str().unwrap_or_default().to_string(),
                        });
                    }
                });
            return Ok(institutions);
        }
        Ok(Vec::new())
    }

    pub async fn get_institution_detail(
        &self,
        key: String,
        place_id: String,
    ) -> Result<GetInstitutionDetailItem, ApiError> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/place/details/json?key={}&placeid={}",
            key, place_id
        );
        let mut country = None;
        let mut city = None;
        let mut postal_code = None;
        let mut name = None;
        let mut formatted_address = None;
        let mut international_phone_number = None;
        if let Ok(response) = reqwest::get(url).await {
            let json: Value = response.json().await.unwrap_or_default();
            let result = &json["result"];
            let empty_vec = Vec::new();
            let address_components = result["address_components"]
                .as_array()
                .unwrap_or(&empty_vec);

            for component in address_components {
                let empty_vec = Vec::new();
                let types = component["types"].as_array().unwrap_or(&empty_vec);
                if types.iter().any(|t| t == "country") {
                    country = component["long_name"].as_str().map(|t| t.to_string());
                }
                if types.iter().any(|t| t == "locality" || t == "postal_town") {
                    city = component["long_name"].as_str().map(|t| t.to_string());
                }
                if types.iter().any(|t| t == "postal_code") {
                    postal_code = component["long_name"].as_str().map(|t| t.to_string());
                }
            }

            name = result["name"].as_str().map(|t| t.to_string());
            formatted_address = result["formatted_address"].as_str().map(|t| t.to_string());
            international_phone_number = result["international_phone_number"]
                .as_str()
                .map(|t| t.to_string());
        }
        return Ok(GetInstitutionDetailItem {
            name: name.unwrap_or_default(),
            formatted_address: formatted_address.unwrap_or_default(),
            place_id,
            country,
            city,
            postal_code,
            international_phone_number,
        });
    }

    // pub async fn get_degree(
    //     &self,
    //     degree: String,
    //     is_available: Option<i16>,
    // ) -> Result<GetDegreeResponse, ApiError> {
    //     Ok(GetDegreeResponse {
    //         degree_list: self.util_repo.get_degree(&degree, is_available).await,
    //     })
    // }

    // pub async fn get_degrees_by_ids(&self, ids: &Vec<Uuid>) -> Result<Vec<Degree>, ApiError> {
    //     Ok(self.util_repo.get_degrees_by_ids(ids).await)
    // }

    // pub async fn update_degree(
    //     &self,
    //     id: Uuid,
    //     degree_name: String,
    //     abbreviation: String,
    //     is_available: i16,
    // ) -> Result<bool, ApiError> {
    //     Ok(self
    //         .util_repo
    //         .update_degree(id, &degree_name, &abbreviation, is_available)
    //         .await)
    // }

    pub async fn get_categories(
        &self,
        name: String,
        is_available: Option<bool>,
        start: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<Category>, ApiError> {
        Ok(self
            .util_repo
            .get_categories(&name, is_available, start.unwrap_or(0), limit.unwrap_or(10))
            .await)
    }

    pub async fn get_category_by_id(&self, id: &str) -> Result<Category, ApiError> {
        let id = Uuid::from_str(id)
            .map_err(|_| ApiError::DbError(DbError::Str("Invalid UUID format".to_string())))?;
        if let Some(category) = self.util_repo.get_category_by_id(id).await {
            Ok(category)
        } else {
            Err(DbError::Str("This category does not exist.".to_string()))?
        }
    }

    pub async fn insert_categories(&self, categories: Vec<String>) -> Vec<Uuid> {
        let mut ids: Vec<Uuid> = Vec::new();
        for category in categories {
            if let Some(c) = self.util_repo.get_category_by_name(category.trim()).await {
                ids.push(c.id);
            } else if let Some(c) = self.util_repo.insert_category(category.trim()).await {
                ids.push(c.id);
            }
        }
        ids
    }

    pub async fn update_category(
        &self,
        id: Uuid,
        name: &str,
        is_available: bool,
    ) -> Result<bool, ApiError> {
        Ok(self.util_repo.update_category(id, name, is_available).await)
    }

    pub async fn delete_category(&self, id: Uuid) -> Result<bool, ApiError> {
        Ok(self.util_repo.delete_category(id).await)
    }

    pub async fn get_last_block_number(&self) -> Result<Option<String>, ApiError> {
        self.util_repo
            .get_last_block_number()
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }

    pub async fn upsert_last_block_number(
        &self,
        last_block_number: &str,
    ) -> Result<String, ApiError> {
        self.util_repo
            .upsert_last_block_number(last_block_number)
            .await
            .map_err(|err| DbError::Str(err.to_string()).into())
    }
}
