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
