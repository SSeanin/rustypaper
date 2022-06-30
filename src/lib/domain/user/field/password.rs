use crate::domain::Result;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self> {
        match validator::validate_length(&password, Some(8), None, None) {
            true => Ok(Self(password)),
            false => {
                let mut error = validator::ValidationError::new("length");
                error.message = Some(Cow::from("Password must be longer than 8 characters"));
                error.add_param(Cow::from("password"), &password);
                Err(error)?
            }
        }
    }

    pub fn to_hashed(&self) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Argon2::default()
            .hash_password(self.0.as_bytes(), &salt)?
            .to_string())
    }

    pub fn verify(password: &str, hash: &str) -> Result<()> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
