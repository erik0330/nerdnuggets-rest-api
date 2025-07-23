use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct AppleUserResult {
    pub sub: String, // Apple's unique user ID
    pub email: Option<String>,
    pub email_verified: Option<String>,
    pub is_private_email: Option<String>,
    pub name: Option<AppleName>,
}

#[derive(Debug, Deserialize)]
pub struct AppleName {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
struct AppleClientSecret {
    iss: String, // Team ID
    iat: u64,    // Issued at
    exp: u64,    // Expiration time
    aud: String, // Audience (Apple)
    sub: String, // Subject (Client ID)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AppleTokenResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<i64>,
    refresh_token: Option<String>,
    id_token: Option<String>,
}

pub async fn get_apple_user_with_code(
    authorization_code: &str,
    client_id: &str,
    team_id: &str,
    key_id: &str,
    private_key: &str,
) -> Result<AppleUserResult, Box<dyn Error>> {
    let client_secret = generate_client_secret(client_id, team_id, key_id, private_key)?;

    let client = Client::new();
    let response = client
        .post("https://appleid.apple.com/auth/token")
        .form(&[
            ("client_id", client_id),
            ("client_secret", &client_secret),
            ("code", authorization_code),
            ("grant_type", &"authorization_code".to_string()),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("Apple token exchange failed: {}", error_text).into());
    }

    let token_response: AppleTokenResponse = response.json().await?;

    // Extract the identity token and decode it to get user info
    if let Some(id_token) = token_response.id_token {
        decode_apple_id_token(&id_token)
    } else {
        Err("No ID token received from Apple".into())
    }
}

fn decode_apple_id_token(id_token: &str) -> Result<AppleUserResult, Box<dyn Error>> {
    // In a production environment, you should verify the JWT signature
    // using Apple's public keys. For now, we'll decode without verification.

    // Split the JWT token
    let parts: Vec<&str> = id_token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT token format".into());
    }

    // Decode the payload (second part)
    let payload = parts[1];
    #[allow(deprecated)]
    let decoded = base64::decode(payload)?;
    let payload_str = String::from_utf8(decoded)?;

    // Parse the JSON payload
    let user_data: serde_json::Value = serde_json::from_str(&payload_str)?;

    Ok(AppleUserResult {
        sub: user_data["sub"].as_str().unwrap_or("unknown").to_string(),
        email: user_data["email"].as_str().map(|s| s.to_string()),
        email_verified: user_data["email_verified"].as_str().map(|s| s.to_string()),
        is_private_email: user_data["is_private_email"]
            .as_str()
            .map(|s| s.to_string()),
        name: None, // Name is only provided on first login
    })
}

fn generate_client_secret(
    client_id: &str,
    team_id: &str,
    key_id: &str,
    private_key: &str,
) -> Result<String, Box<dyn Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let claims = AppleClientSecret {
        iss: team_id.to_string(),
        iat: now,
        exp: now + 15777000, // 6 months
        aud: "https://appleid.apple.com".to_string(),
        sub: client_id.to_string(),
    };

    let header = Header {
        alg: jsonwebtoken::Algorithm::ES256,
        kid: Some(key_id.to_string()),
        ..Default::default()
    };

    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_ec_pem(private_key.as_bytes())?,
    )?;

    Ok(token)
}

// Alternative method for handling Apple Sign-In with identity token directly
pub async fn verify_apple_token(
    identity_token: &str,
    _client_id: &str,
    _team_id: &str,
    _key_id: &str,
    _private_key: &str,
) -> Result<AppleUserResult, Box<dyn Error>> {
    // This would verify the JWT signature using Apple's public keys
    // For now, we'll just decode the token
    decode_apple_id_token(identity_token)
}
