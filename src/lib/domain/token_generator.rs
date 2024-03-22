use crate::{
    data::Id,
    domain::{datetime::AppDatetime, Result},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// convert 5 minutes to ms
pub const ACCESS_TOKEN_EXP: usize = 5 * 60 * 1000;
// convert 7 days to ms
pub const REFRESH_TOKEN_EXP: usize = 7 * 24 * 60 * 60 * 1000;

pub trait Claim {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub jti: String,
}

impl Claim for AccessTokenClaims {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub sid: String,
}

impl Claim for RefreshTokenClaims {}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct TokenGenerator {
    secret: String,
}

impl TokenGenerator {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_token_pair(&self, sub: &str) -> Result<TokenPair> {
        let id = Id::new().into_inner().to_string();

        let access_token_claims = AccessTokenClaims {
            sub: sub.to_owned(),
            iat: AppDatetime::now().timestamp_millis() as usize,
            exp: AppDatetime::now().timestamp_millis() as usize + ACCESS_TOKEN_EXP,
            jti: id.clone(),
        };

        let access_token = encode(
            &Header::new(Algorithm::HS512),
            &access_token_claims,
            &EncodingKey::from_base64_secret(self.secret.as_ref()),
        )?;

        let refresh_token_claims = RefreshTokenClaims {
            sub: sub.to_owned(),
            iat: AppDatetime::now().timestamp_millis() as usize,
            exp: AppDatetime::now().timestamp_millis() as usize + REFRESH_TOKEN_EXP,
            sid: id,
        };

        let refresh_token = encode(
            &Header::new(Algorithm::HS512),
            &refresh_token_claims,
            &EncodingKey::from_base64_secret(self.secret.as_ref()),
        )?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub fn verify_token<C>(&self, token: &str) -> Result<C>
    where
        C: Claim + DeserializeOwned,
    {
        let validation = Validation::new(Algorithm::HS512);

        Ok(decode::<C>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )?
        .claims)
    }
}
