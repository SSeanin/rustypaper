use crate::data::Id;
use crate::domain::datetime::AppDatetime;
use crate::domain::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

// convert 5 minutes to ms
pub const ACCESS_TOKEN_EXP: usize = 5 * 60 * 1000;
// convert 7 days to ms
pub const REFRESH_TOKEN_EXP: usize = 7 * 24 * 60 * 60 * 1000;

pub trait Claim {}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    exp: usize,
    iat: usize,
    sub: String,
    jti: String,
}

impl Claim for AccessTokenClaims {}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    exp: usize,
    iat: usize,
    sub: String,
    sid: String,
}

impl Claim for RefreshTokenClaims {}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    access_token: String,
    refresh_token: String,
}

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
            &EncodingKey::from_secret(self.secret.as_ref()),
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
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub fn verify_token<C>(&self, token: &str, sub: &str) -> Result<C>
    where
        C: Claim + DeserializeOwned,
    {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.sub = Some(sub.to_owned());

        Ok(decode::<C>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )?
        .claims)
    }
}
