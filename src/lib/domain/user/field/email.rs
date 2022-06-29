use crate::domain::Result;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self> {
        match validator::validate_email(&email) {
            true => Ok(Self(email)),
            false => {
                let mut error = validator::ValidationError::new("invalid");
                error.message = Some(Cow::from("Not a valid email value"));
                error.add_param(Cow::from("email"), &email);
                Err(error)?
            }
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}