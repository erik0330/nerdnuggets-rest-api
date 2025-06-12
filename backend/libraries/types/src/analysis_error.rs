use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PublicationInfo {
    pub date: Value,
    pub journal: Value,
    pub keywords: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Metadata {
    pub title: String,
    pub authors: Option<Vec<String>>,
    #[serde(default)]
    pub paper_id: i64,
    pub paper_link: Value,
    pub publication_info: PublicationInfo,
    pub institution_links: Value,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PaperSummary {
    #[serde(default)]
    pub summary: Value,
    pub metadata: Metadata,
    #[serde(default)]
    pub technical_assessment: Value,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Analysis {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(deserialize_with = "deserialize_counts")]
    pub counts: i16,
    pub findings: Value,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PaperAnalysis {
    pub summary: Value,
    pub analysis: Vec<Analysis>,
    pub metadata: Value,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ArticleData {
    #[serde(rename = "abstract")]
    pub _abstract: String,
    pub introduction: String,
    pub conclusion: String,
    pub article: String,
    pub references: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AnalysisError {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub output_tokens: i64,
    #[serde(default)]
    pub total_cost: f32,
    #[serde(rename = "paperSummary")]
    pub paper_summary: PaperSummary,
    #[serde(default, rename = "paperAnalysis")]
    pub paper_analysis: PaperAnalysis,
    #[serde(default, rename = "articleData")]
    pub article_data: ArticleData,
    pub created_at: Option<DateTime<Utc>>,
}

fn deserialize_counts<'de, D>(deserializer: D) -> Result<i16, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};

    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(num) => num.as_i64().map(|n| n as i16).ok_or_else(|| {
            D::Error::invalid_type(Unexpected::Str(&num.to_string()), &"a valid i16")
        }),

        serde_json::Value::String(s) => s.parse::<i16>().map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a valid i16 number string")
        }),

        _ => Err(D::Error::custom(
            "Invalid type for counts field, expected integer or string",
        )),
    }
}
