use crate::domain::Result;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, From)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self> {
        match validator::validate_length(&password, Some(8), None, None) {
            true => {
                let salt = SaltString::generate(&mut OsRng);
                Ok(Self(
                    Argon2::default()
                        .hash_password(password.as_bytes(), &salt)?
                        .to_string(),
                ))
            }
            false => {
                let mut error = validator::ValidationError::new("length");
                error.message = Some(Cow::from("Password must be longer than 8 characters"));
                error.add_param(Cow::from("password"), &password);
                Err(error)?
            }
        }
    }

    pub fn verify(&self, password: &str) -> Result<()> {
        Ok(Argon2::default().verify_password(password.as_bytes(), &PasswordHash::new(&self.0)?)?)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for Password {
    fn default() -> Self {
        Self("".to_owned())
    }
}
