use chrono::{DateTime, Duration, Utc};
use dotenv;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Env {
    pub port: u32,
    pub jwt_secret: String,
    pub jwt_ttl_in_minutes: i64,
    pub database_url: String,
    pub database_max_connections: u32,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_bucket_name: String,
    pub email_verify_exp_second: i64,
    pub email_verify_limit: i16,
    pub email_region: String,
    pub frontend_url: String,
    pub vapid_private_pem: String,
    pub production: bool,
    pub ai_backend_url: String,
    pub google_map_api_key: String,
    pub evm_job_schedule: String,
    pub dao_contract_address: String,
    pub funding_contract_address: String,
    pub rpc_url: String,
    pub chain_id: u64,
    pub wallet_private_key: String,
    pub dao_duration: Duration,
    pub apple_client_id: String,
    pub apple_team_id: String,
    pub apple_key_id: String,
    pub apple_private_key: String,
}

impl Env {
    pub fn init() -> Self {
        dotenv::dotenv().ok();
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000);

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_ttl_in_minutes = std::env::var("JWT_TTL_IN_MINUTES")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(30);

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(10);

        let aws_access_key_id =
            std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
        let aws_secret_access_key =
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
        let aws_bucket_name =
            std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set");

        let email_verify_exp_second = std::env::var("EMAIL_VERIFY_EXP_SECOND")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(100);
        let email_verify_limit = std::env::var("EMAIL_VERIFY_LIMIT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5);
        let email_region = std::env::var("EMAIL_REGION").expect("EMAIL_REGION must be set");

        let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

        let vapid_private_pem =
            std::env::var("VAPID_PRIVATE_PEM").expect("VAPID_PRIVATE_PEM must be set");

        let production = std::env::var("PRODUCTION")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or_default();

        let ai_backend_url = std::env::var("AI_BACKEND_URL").expect("AI_BACKEND_URL must be set");

        let google_map_api_key =
            std::env::var("GOOGLE_MAP_API_KEY").expect("GOOGLE_MAP_API_KEY must be set");

        let evm_job_schedule =
            std::env::var("EVM_JOB_SCHEDULE").expect("EVM_JOB_SCHEDULE must be set");
        let dao_contract_address =
            std::env::var("DAO_CONTRACT_ADDRESS").expect("DAO_CONTRACT_ADDRESS must be set");
        let funding_contract_address = std::env::var("FUNDING_CONTRACT_ADDRESS")
            .expect("FUNDING_CONTRACT_ADDRESS must be set");
        let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set");
        let chain_id = std::env::var("CHAIN_ID")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(1);
        let wallet_private_key =
            std::env::var("WALLET_PRIVATE_KEY").expect("WALLET_PRIVATE_KEY must be set");
        let dao_duration = if production {
            Duration::days(7)
        } else {
            Duration::hours(2)
        };

        let apple_client_id =
            std::env::var("APPLE_CLIENT_ID").expect("APPLE_CLIENT_ID must be set");
        let apple_team_id = std::env::var("APPLE_TEAM_ID").expect("APPLE_TEAM_ID must be set");
        let apple_key_id = std::env::var("APPLE_KEY_ID").expect("APPLE_KEY_ID must be set");
        let apple_private_key =
            std::env::var("APPLE_PRIVATE_KEY").expect("APPLE_PRIVATE_KEY must be set");

        Self {
            port,
            jwt_secret,
            jwt_ttl_in_minutes,
            database_url,
            database_max_connections,
            aws_access_key_id,
            aws_secret_access_key,
            aws_bucket_name,
            email_verify_exp_second,
            email_verify_limit,
            email_region,
            frontend_url,
            vapid_private_pem,
            production,
            ai_backend_url,
            google_map_api_key,
            evm_job_schedule,
            dao_contract_address,
            funding_contract_address,
            rpc_url,
            chain_id,
            wallet_private_key,
            dao_duration,
            apple_client_id,
            apple_team_id,
            apple_key_id,
            apple_private_key,
        }
    }

    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    pub fn generate_passkey(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(100_000..1_000_000)
    }
}
