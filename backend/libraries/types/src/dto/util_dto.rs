use crate::models::{City, Country, Degree, EmploymentsInfo, HashTagsInfo, WallPapers};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetCountryOption {
    pub country: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetCountryResponse {
    pub country_list: Vec<Country>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetCityOption {
    pub country: Option<String>,
    pub city: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstitutionsOption {
    pub search_type: String,
    pub query: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstitutionsItem {
    pub name: String,
    pub formatted_address: String,
    pub place_id: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstitutionDetailOption {
    pub place_id: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstitutionDetailItem {
    pub name: String,
    pub formatted_address: String,
    pub place_id: String,
    pub country: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub international_phone_number: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetCityResponse {
    pub city_list: Vec<City>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
pub struct GetDegreeOption {
    pub degree: Option<String>,
    pub is_available: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryOption {
    pub name: Option<String>,
    pub is_available: Option<bool>,
    pub start: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDegreeOption {
    pub id: Uuid,
    pub degree_name: String,
    pub abbreviation: String,
    pub is_available: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDegreeResponse {
    pub degree_list: Vec<Degree>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHashTagsOption {
    #[serde(rename = "q")]
    pub hashtags: Option<String>,
    pub limit: Option<i32>,
    pub order_by: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHashTagsResponse {
    pub hashtags: Vec<HashTagsInfo>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEmploymentsOption {
    pub employments: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEmploymentsResponse {
    pub employments: Vec<EmploymentsInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWallPapersResponse {
    pub wallpapers: Vec<WallPapers>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFileFromS3Request {
    pub link: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtractProjectInfoRequest {
    #[validate(url)]
    pub s3_paper: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NerdNuggetsInfo {
    pub title: String,
    pub description: String,
    pub research_objectives: Vec<String>,
    pub methodology: String,
    pub expected_outcomes: String,
    pub strengths_potential_limitations: StrengthsLimitations,
    pub commercial_applications: String,
    pub societal_benefit: String,
    pub risk_assessment: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StrengthsLimitations {
    pub strengths: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtractProjectInfoResponse {
    pub status: String,
    pub data: NerdNuggetsInfo,
}
