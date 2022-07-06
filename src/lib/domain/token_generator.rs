use crate::data::Id;
use crate::domain::datetime::AppDatetime;
use crate::domain::{DomainError, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccessTokenClaims {
    type Error = DomainError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let access_token = match request.cookies().get_private("Authorization") {
            None => {
                return Outcome::Failure((Status::Unauthorized, DomainError::InvalidToken));
            }
            Some(access_token_cookie) => access_token_cookie.value().to_owned(),
        };

        let token_generator = match request.rocket().state::<TokenGenerator>() {
            None => {
                return Outcome::Failure((Status::InternalServerError, DomainError::TokenGenerator))
            }
            Some(token_generator) => token_generator,
        };

        match token_generator.verify_token::<AccessTokenClaims>(access_token.as_str()) {
            Err(e) => {
                return Outcome::Failure((Status::Unauthorized, e));
            }
            Ok(claims) => Outcome::Success(claims),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub sid: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RefreshTokenClaims {
    type Error = DomainError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let refresh_token = match request.headers().get_one("refresh-token") {
            None => {
                return Outcome::Failure((Status::Unauthorized, DomainError::InvalidToken));
            }
            Some(refresh_token) => {
                let token = refresh_token.split_whitespace().collect::<Vec<&str>>();

                if token[0] == "Bearer" {
                    token[1]
                } else {
                    return Outcome::Failure((Status::Unauthorized, DomainError::InvalidToken));
                }
            }
        };

        let token_generator = match request.rocket().state::<TokenGenerator>() {
            None => {
                return Outcome::Failure((Status::InternalServerError, DomainError::TokenGenerator))
            }
            Some(token_generator) => token_generator,
        };

        match token_generator.verify_token::<RefreshTokenClaims>(refresh_token) {
            Err(e) => {
                return Outcome::Failure((Status::Unauthorized, e));
            }
            Ok(claims) => Outcome::Success(claims),
        }
    }
}

impl Claim for RefreshTokenClaims {}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
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
