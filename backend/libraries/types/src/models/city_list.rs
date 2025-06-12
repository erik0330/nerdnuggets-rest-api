use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct CityList {
    pub id: Uuid,
    pub city: String,
    pub country: String,
    pub iso2: String,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Country {
    pub country: String,
    pub iso2: String,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct City {
    pub city: String,
}
