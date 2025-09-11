use base64;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest;
use serde::{Deserialize, Serialize};
use types::{
    dto::{PrivyTokenClaimsDto, PrivyUserInfo},
    error::{ApiError, TokenError, UserError},
};
use utils::env::Env;

#[derive(Clone)]
pub struct PrivyService {
    app_id: String,
    app_secret: String,
    jwks_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivyJWKS {
    keys: Vec<PrivyJWK>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivyJWK {
    kty: String,
    kid: String,
    // use_: String,
    #[serde(rename = "use")]
    key_use: Option<String>,
    x: String,
    y: String,
    crv: String,
    alg: String,
}

impl PrivyService {
    pub fn new(env: &Env) -> Self {
        Self {
            app_id: env.privy_app_id.clone(),
            app_secret: env.privy_app_secret.clone(),
            jwks_url: format!(
                "https://auth.privy.io/api/v1/apps/{}/jwks.json",
                env.privy_app_id
            ),
        }
    }

    pub async fn verify_privy_token(&self, token: &str) -> Result<PrivyTokenClaimsDto, ApiError> {
        // First, get the JWKS from Privy
        let jwks = self.fetch_jwks().await?;

        // Decode the JWT header to get the key ID
        let header = jsonwebtoken::decode_header(token)
            .map_err(|_| TokenError::InvalidToken(token.to_string()))?;

        let kid = header
            .kid
            .ok_or_else(|| TokenError::InvalidToken("Missing key ID".to_string()))?;

        // Find the matching key
        let jwk = jwks
            .keys
            .iter()
            .find(|key| key.kid == kid)
            .ok_or_else(|| TokenError::InvalidToken("Key not found".to_string()))?;

        // Validate the JWK
        if jwk.crv != "P-256" {
            return Err(ApiError::UserError(UserError::Str(
                "Unsupported curve type".to_string(),
            )));
        }

        if jwk.alg != "ES256" {
            return Err(ApiError::UserError(UserError::Str(
                "Unsupported algorithm".to_string(),
            )));
        }

        // First, let's decode without validation to see the actual issuer
        let decoding_key = DecodingKey::from_ec_components(&jwk.x, &jwk.y)
            .map_err(|e| TokenError::InvalidToken(e.to_string()))?;

        // Decode without validation to inspect the claims
        let mut validation_no_check = Validation::new(Algorithm::ES256);
        validation_no_check.validate_aud = false;
        validation_no_check.validate_exp = false;

        let token_data_debug =
            decode::<PrivyTokenClaimsDto>(token, &decoding_key, &validation_no_check)
                .map_err(|e| TokenError::InvalidToken(e.to_string()))?;

        // Now verify the token with proper validation
        let mut validation = Validation::new(Algorithm::ES256);
        validation.set_audience(&[&self.app_id]);

        // Set the issuer based on what we found in the token
        validation.set_issuer(&[&token_data_debug.claims.iss]);

        let token_data = decode::<PrivyTokenClaimsDto>(token, &decoding_key, &validation).map_err(
            |e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => TokenError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                    TokenError::InvalidToken(format!(
                        "Invalid issuer: expected one of {:?}, got {}",
                        &["https://auth.privy.io"],
                        token_data_debug.claims.iss
                    ))
                }
                _ => TokenError::InvalidToken(e.to_string()),
            },
        )?;

        Ok(token_data.claims)
    }

    async fn fetch_jwks(&self) -> Result<PrivyJWKS, ApiError> {
        let client = reqwest::Client::new();
        let response =
            client.get(&self.jwks_url).send().await.map_err(|_| {
                ApiError::UserError(UserError::Str("Failed to fetch JWKS".to_string()))
            })?;

        if !response.status().is_success() {
            return Err(ApiError::UserError(UserError::Str(
                "Failed to fetch JWKS".to_string(),
            )));
        }

        let jwks: PrivyJWKS = response
            .json()
            .await
            .map_err(|_| ApiError::UserError(UserError::Str("Failed to parse JWKS".to_string())))?;

        Ok(jwks)
    }

    #[allow(deprecated)]
    pub async fn get_user_info(&self, user_id: &str) -> Result<PrivyUserInfo, ApiError> {
        let client = reqwest::Client::new();
        let url = format!("https://api.privy.io/v1/users/{}", user_id);

        // Create Basic Auth header with app_id:app_secret
        let credentials = base64::encode(format!("{}:{}", self.app_id, self.app_secret));
        let auth_header = format!("Basic {}", credentials);

        let response = client
            .get(&url)
            .header("Authorization", auth_header)
            .header("privy-app-id", &self.app_id)
            .send()
            .await
            .map_err(|e| {
                ApiError::UserError(UserError::Str(format!("Failed to fetch user info: {}", e)))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::UserError(UserError::Str(format!(
                "Failed to fetch user info: {} - {}",
                status, error_text
            ))));
        }

        let user_info: PrivyUserInfo = response.json().await.map_err(|e| {
            ApiError::UserError(UserError::Str(format!("Failed to parse user info: {}", e)))
        })?;

        Ok(user_info)
    }
}
