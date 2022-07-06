use crate::domain::token_generator::{AccessTokenClaims, RefreshTokenClaims};

pub struct LoginObject {
    pub email: String,
    pub password: String,
}

pub struct RefreshObject {
    pub access_token_claims: AccessTokenClaims,
    pub refresh_token_claims: RefreshTokenClaims,
}
