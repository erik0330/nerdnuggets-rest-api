use anyhow::anyhow;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::providers::Provider;
use std::sync::Arc;

abigen!(DAO_CONTRACT, "./abis/dao_contract_abi.json");
abigen!(FUNDING_CONTRACT, "./abis/funding_contract_abi.json");

#[derive(Clone)]
pub struct EVMClient {
    dao_contract: DAO_CONTRACT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    funding_contract: FUNDING_CONTRACT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    provider: Provider<Http>,
}

impl EVMClient {
    pub fn init(
        dao_contract_address: &str,
        funding_contract_address: &str,
        wallet_private_key: &str,
        rpc_url: &str,
        chain_id: u64,
    ) -> EVMClient {
        let dao_contract_address = dao_contract_address
            .parse::<Address>()
            .expect("Invalid DAO_CONTRACT_ADDRESS");
        let funding_contract_address = funding_contract_address
            .parse::<Address>()
            .expect("Invalid FUNDING_CONTRACT_ADDRESS");
        let provider = Provider::<Http>::try_from(rpc_url).expect("Invalid RPC_URL");
        let wallet: LocalWallet = wallet_private_key
            .parse()
            .expect("Invalid WALLET_PRIVATE_KEY");
        let wallet = wallet.with_chain_id(chain_id);
        let provider_with_wallet = SignerMiddleware::new(provider.clone(), wallet);

        let dao_contract =
            DAO_CONTRACT::new(dao_contract_address, Arc::new(provider_with_wallet.clone()));
        let funding_contract = FUNDING_CONTRACT::new(
            funding_contract_address,
            Arc::new(provider_with_wallet.clone()),
        );

        EVMClient {
            dao_contract,
            funding_contract,
            provider,
        }
    }

    pub async fn create_project(
        &self,
        proposal_id: u64,
        researcher_address: &str,
        milestone_data: Vec<(u64, u64)>,
        metadata_url: String,
    ) -> Result<String, anyhow::Error> {
        let proposal_id = U256::from(proposal_id);
        let researcher_address = researcher_address.parse::<Address>()?;

        let milestones = milestone_data
            .into_iter()
            .map(|(duration, goal)| dao_contract::Milestone {
                duration: U256::from(duration),
                funding_goal: U256::from(goal),
                prediction_deadline: U256::from(duration),
            })
            .collect::<Vec<dao_contract::Milestone>>();

        if let Some(tx) = self
            .dao_contract
            .create_project(proposal_id, researcher_address, milestones, metadata_url)
            .send()
            .await?
            .await?
        {
            return Ok(format!("{:?}", tx.transaction_hash));
        } else {
            return Err(anyhow!("Unexpected error"));
        }
    }

    pub async fn approve_project(&self, proposal_id: u64) -> Result<String, anyhow::Error> {
        let proposal_id = U256::from(proposal_id);
        if let Some(tx) = self
            .dao_contract
            .approve_project(proposal_id)
            .send()
            .await?
            .await?
        {
            return Ok(tx.transaction_hash.to_string());
        } else {
            return Err(anyhow!("Unexpected error"));
        }
    }

    pub async fn get_dao_contract_events(
        &self,
        from_block_number: Option<u64>,
        to_block_number: Option<u64>,
    ) -> Result<(Vec<DAO_CONTRACTEvents>, Option<u64>), anyhow::Error> {
        let events = self.dao_contract.events();
        let to_block_number = to_block_number.unwrap_or(
            self.provider
                .get_block(BlockNumber::Latest)
                .await?
                .unwrap()
                .number
                .unwrap()
                .as_u64(),
        );
        let from_block_number = from_block_number.unwrap_or(to_block_number - 10_000);
        let filtered_events = events
            .from_block(from_block_number)
            .to_block(to_block_number)
            .query()
            .await?;

        Ok((filtered_events, Some(to_block_number)))
    }

    pub async fn get_funding_contract_events(
        &self,
        from_block_number: Option<u64>,
        to_block_number: Option<u64>,
    ) -> Result<(Vec<FUNDING_CONTRACTEvents>, Option<u64>), anyhow::Error> {
        let events = self.funding_contract.events();
        let to_block_number = to_block_number.unwrap_or(
            self.provider
                .get_block(BlockNumber::Latest)
                .await?
                .unwrap()
                .number
                .unwrap()
                .as_u64(),
        );
        let from_block_number = from_block_number.unwrap_or(to_block_number - 10_000);
        let filtered_events = events
            .from_block(from_block_number)
            .to_block(to_block_number)
            .query()
            .await?;

        Ok((filtered_events, Some(to_block_number)))
    }
}
