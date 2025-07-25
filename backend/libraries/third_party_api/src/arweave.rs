use arrs::wallet::ArWallet;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize)]
struct TransactionData {
    id: String,
}

pub async fn upload_metadata_to_arweave(
    tags: &Vec<(String, String)>,
    content: &str,
) -> Result<String, anyhow::Error> {
    let jwk = read_to_string("./arweave_pk.json").unwrap();
    let mut arwallet = ArWallet::from_jwk(&jwk);

    let mut tx = arwallet.create_data_transaction(content.as_bytes()).await;
    for (k, v) in tags.iter() {
        tx.add_tag(k, v);
    }
    tx.sign();
    let transaction_data: TransactionData = serde_json::from_str(&tx.tx_data_json()).unwrap();
    tx.submit().await.ok();

    tokio::spawn(async move {
        while arwallet.uploader().current_idx() < arwallet.uploader().chunk_size() {
            arwallet.upload().await.ok();
        }
    });

    Ok(transaction_data.id)
}

/// Helper function to upload bounty-related metadata to Arweave with standardized tags
pub async fn upload_metadata(
    content_type: &str,
    metadata: &serde_json::Value,
    additional_tags: Option<Vec<(String, String)>>,
) -> Result<String, anyhow::Error> {
    let mut tags = vec![
        ("Content-Type".to_string(), "application/json".to_string()),
        ("App-Name".to_string(), "NerdNuggets".to_string()),
        ("Type".to_string(), content_type.to_string()),
        ("Timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
    ];

    // Add any additional tags
    if let Some(additional) = additional_tags {
        tags.extend(additional);
    }

    upload_metadata_to_arweave(&tags, &serde_json::to_string(metadata).unwrap()).await
}

/// Helper function to upload bounty creation metadata
pub async fn upload_bounty_creation(
    bounty_id: &str,
    nerd_id: &str,
    user_id: &str,
    payload: &serde_json::Value,
    bounty_info: &serde_json::Value,
    user_info: &serde_json::Value,
) -> Result<String, anyhow::Error> {
    let additional_tags = vec![
        ("Bounty-Id".to_string(), bounty_id.to_string()),
        ("Nerd-Id".to_string(), nerd_id.to_string()),
        ("User-Id".to_string(), user_id.to_string()),
    ];

    let metadata = serde_json::json!({
        "payload": payload,
        "bounty": bounty_info,
        "user": user_info,
    });

    upload_metadata("Bounty", &metadata, Some(additional_tags)).await
}

/// Helper function to upload project submission metadata
pub async fn upload_project_submission(
    project_id: &str,
    nerd_id: &str,
    user_id: &str,
    project_info: &serde_json::Value,
) -> Result<String, anyhow::Error> {
    let additional_tags = vec![
        ("Project-Id".to_string(), project_id.to_string()),
        ("Nerd-Id".to_string(), nerd_id.to_string()),
        ("User-Id".to_string(), user_id.to_string()),
    ];

    let metadata = serde_json::json!({
        "action": "submit_project",
        "project_id": project_id,
        "nerd_id": nerd_id,
        "user_id": user_id,
        "project_info": project_info,
        "submitted_at": chrono::Utc::now(),
    });

    upload_metadata("Project", &metadata, Some(additional_tags)).await
}

/// Helper function to upload milestone update metadata
pub async fn upload_milestone_update(
    milestone_id: &str,
    project_id: &str,
    nerd_id: &str,
    payload: &serde_json::Value,
) -> Result<String, anyhow::Error> {
    let additional_tags = vec![
        ("Milestone-Id".to_string(), milestone_id.to_string()),
        ("Project-Id".to_string(), project_id.to_string()),
        ("Nerd-Id".to_string(), nerd_id.to_string()),
    ];

    let metadata = serde_json::json!({
        "action": "update_milestone",
        "milestone_id": milestone_id,
        "project_id": project_id,
        "nerd_id": nerd_id,
        "payload": payload,
    });

    upload_metadata("Milestone", &metadata, Some(additional_tags)).await
}
