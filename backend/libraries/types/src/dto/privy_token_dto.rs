use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PrivyTokenClaimsDto {
    pub sub: String, // Privy user ID
    pub iat: i64,
    pub exp: i64,
    pub aud: String, // Audience (your app ID)
    pub iss: String, // Issuer (Privy)
                     // pub email: Option<String>,
                     // pub phone: Option<String>,
                     // pub linked_accounts: Option<Vec<LinkedAccount>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LinkedAccount {
    pub r#type: String, // "email", "farcaster", "passkey", "google_oauth", "apple_oauth", "wallet", etc.
    pub address: Option<String>, // For email and wallet accounts
    pub email: Option<String>, // For OAuth accounts
    pub wallet_client: Option<String>, // For wallet accounts

    // Farcaster specific fields
    pub fid: Option<u64>,
    pub owner_address: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub profile_picture_url: Option<String>,

    // Passkey specific fields
    pub credential_id: Option<String>,
    pub authenticator_name: Option<String>,
    pub public_key: Option<String>,
    pub created_with_browser: Option<String>,
    pub created_with_os: Option<String>,
    pub created_with_device: Option<String>,
    pub enrolled_in_mfa: Option<bool>,

    // Common verification fields
    pub verified_at: Option<u64>,
    pub first_verified_at: Option<u64>,
    pub latest_verified_at: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MfaMethod {
    pub r#type: String,
    pub verified_at: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PrivyUserInfo {
    pub id: String,
    pub created_at: u64,
    pub linked_accounts: Vec<LinkedAccount>,
    pub mfa_methods: Vec<MfaMethod>,
    pub has_accepted_terms: bool,
    pub is_guest: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PrivyLoginRequest {
    pub id_token: String,
}
