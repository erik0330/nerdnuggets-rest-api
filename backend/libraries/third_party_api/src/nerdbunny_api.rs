use reqwest::Client;
use serde::{Deserialize, Serialize};
use types::dto::{
    ExtractProjectInfoRequest, ExtractProjectInfoResponse, NerdNuggetsInfo, StrengthsLimitations,
};

#[derive(Serialize, Deserialize, Debug)]
struct ExternalApiResponse {
    status: String,
    data: ExternalNerdNuggetsInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalNerdNuggetsInfo {
    title: String,
    description: String,
    research_objectives: Vec<String>,
    methodology: String,
    expected_outcomes: String,
    strengths_potential_limitations: ExternalStrengthsLimitations,
    commercial_applications: String,
    societal_benefit: String,
    risk_assessment: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalStrengthsLimitations {
    strengths: Vec<String>,
    limitations: Vec<String>,
}

pub async fn extract_project_info(
    request: ExtractProjectInfoRequest,
) -> Result<ExtractProjectInfoResponse, anyhow::Error> {
    let client = Client::new();

    let url = format!(
        "https://www.nerdbunny.com/api/v1/papers/integrations/extract_nerdnuggets_info/?s3_paper={}",
        urlencoding::encode(&request.s3_paper)
    );

    let response = client
        .get(&url)
        .header("User-Agent", "NerdNuggets-Backend/2.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "External API request failed with status: {}",
            response.status()
        ));
    }

    println!("response: {:?}", response);

    let external_response: ExternalApiResponse = response.json().await?;

    // Convert external response to our internal format
    let nerdnuggets_info = NerdNuggetsInfo {
        title: external_response.data.title,
        description: external_response.data.description,
        research_objectives: external_response.data.research_objectives,
        methodology: external_response.data.methodology,
        expected_outcomes: external_response.data.expected_outcomes,
        strengths_potential_limitations: StrengthsLimitations {
            strengths: external_response
                .data
                .strengths_potential_limitations
                .strengths,
            limitations: external_response
                .data
                .strengths_potential_limitations
                .limitations,
        },
        commercial_applications: external_response.data.commercial_applications,
        societal_benefit: external_response.data.societal_benefit,
        risk_assessment: external_response.data.risk_assessment,
        tags: external_response.data.tags,
    };

    Ok(ExtractProjectInfoResponse {
        status: external_response.status,
        data: nerdnuggets_info,
    })
}
