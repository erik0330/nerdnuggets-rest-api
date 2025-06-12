use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResearchCheckType {
    #[default]
    ResearchCheck,
    GenerateArticle,
    GetInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResearchCheckSummaryType {
    #[default]
    Basic,
    Advanced,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResearchCheckAdvancedMethod {
    #[default]
    Weight,
    Method,
    Result,
    Limitation,
    Finding,
    Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResearchCheckCitationFormat {
    #[default]
    APA,
    Chicago,
    CSE,
    AIP,
    ACS,
    IEEE,
}
