use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct Paper {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub paper_id: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_cost: f32,
    pub data_analysis_errors: i16,
    pub logical_framework_errors: i16,
    pub math_errors: i16,
    pub methodology_errors: i16,
    pub research_quality_errors: i16,
    pub technical_presentation_errors: i16,
    pub total_errors: i16,
    pub has_error: bool,
    pub has_article: bool,
    pub citation_format: Option<i16>,
    pub created_at: DateTime<Utc>,
}
