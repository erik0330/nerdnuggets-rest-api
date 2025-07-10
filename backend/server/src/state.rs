use chrono::{Duration, Utc};
use database::{AppService, DatabasePool};
use evm::EVMClient;
use std::{collections::HashMap, sync::Arc};
use twitter_v2::{authorization::Oauth2Client, oauth2::PkceCodeVerifier};
use types::NerdNuggetsOAuth2AppName;
use utils::env::Env;
use uuid::Uuid;

#[allow(dead_code)]
pub struct TwitterChallenge {
    pub verifier: PkceCodeVerifier,
    pub redirect_url: String,
    pub twitter_id: String,
    pub user_id: Option<Uuid>,
    pub exp: i64,
}

impl TwitterChallenge {
    #[allow(dead_code)]
    pub fn new(verifier: PkceCodeVerifier, redirect_url: String, user_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        let exp = now
            .checked_add_signed(Duration::seconds(2 * 60))
            .unwrap()
            .timestamp();
        Self {
            verifier,
            redirect_url,
            twitter_id: String::new(),
            user_id,
            exp,
        }
    }
}

#[allow(dead_code)]
pub struct NobleblocksChallenge {
    pub app_name: NerdNuggetsOAuth2AppName,
    pub state: Uuid,
    pub redirect_url: String,
    pub exp: i64,
    pub user_id: Option<Uuid>,
}

impl NobleblocksChallenge {
    #[allow(dead_code)]
    pub fn new(app_name: NerdNuggetsOAuth2AppName, redirect_url: String) -> Self {
        let now = Utc::now();
        let exp = now
            .checked_add_signed(Duration::seconds(2 * 60))
            .unwrap()
            .timestamp();
        Self {
            app_name,
            state: Uuid::new_v4(),
            redirect_url,
            exp,
            user_id: None,
        }
    }

    #[allow(dead_code)]
    pub fn build_querystring(&self) -> String {
        format!(
            "?app_name={:?}&state={}&redirect_url={}",
            self.app_name, self.state, self.redirect_url
        )
    }
}

#[allow(dead_code)]
pub struct OAuth2Ctx {
    pub client: Oauth2Client,
    pub challenges: HashMap<String, TwitterChallenge>,
    pub nobleblocks_challenges: HashMap<String, NobleblocksChallenge>,
}

impl OAuth2Ctx {
    #[allow(dead_code)]
    pub fn remove_expired_challenges(&mut self) {
        let now = Utc::now().timestamp();
        self.challenges.retain(|_, challenge| challenge.exp < now);
        self.nobleblocks_challenges
            .retain(|_, challenge| challenge.exp < now);
    }
}

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
    pub evm: EVMClient,
    pub service: AppService,
    pub s3_client: aws_sdk_s3::Client,
    pub _ses_client: aws_sdk_sesv2::Client,
}

impl AppState {
    pub fn init(
        db: &Arc<DatabasePool>,
        env: Env,
        s3_client: aws_sdk_s3::Client,
        ses_client: aws_sdk_sesv2::Client,
    ) -> Self {
        let evm = EVMClient::init(
            &env.dao_contract_address,
            &env.wallet_private_key,
            &env.rpc_url,
            env.chain_id,
        );
        Self {
            service: AppService::init(db, &env),
            env,
            evm,
            s3_client,
            _ses_client: ses_client,
        }
    }
}
