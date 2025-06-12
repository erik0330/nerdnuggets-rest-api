use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use types::{
    dto::{TokenClaimsDto, TokenReadDto},
    error::TokenError,
    models::User,
    UserRoleType,
};
use utils::env::Env;

#[derive(Clone)]
pub struct TokenService {
    secret: String,
    ttl_in_minutes: i64,
}

impl TokenService {
    pub fn new(env: &Env) -> Self {
        Self {
            secret: env.jwt_secret.clone(),
            ttl_in_minutes: env.jwt_ttl_in_minutes,
        }
    }

    pub fn retrieve_token_claims(
        &self,
        token: &str,
    ) -> jsonwebtoken::errors::Result<TokenData<TokenClaimsDto>> {
        decode::<TokenClaimsDto>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
    }

    pub fn generate_token(&self, user: User, role: UserRoleType) -> Result<String, TokenError> {
        let iat = chrono::Utc::now().timestamp();
        let exp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(self.ttl_in_minutes))
            .unwrap()
            .timestamp();

        let claims = TokenClaimsDto {
            sub: user.id,
            iat,
            exp,
            role,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| TokenError::TokenCreationError(e.to_string()))?;

        Ok(token)
    }
}
