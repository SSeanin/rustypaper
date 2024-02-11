use crate::domain::{DomainError, Result};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};

#[derive(Debug, Deserialize, Serialize)]
pub struct LastName(String);

impl LastName {
    pub fn new(last_name: String) -> Result<Self> {
        match validator::validate_length(&last_name, Some(2), None, None) {
            true => Ok(Self(last_name)),
            false => {
                let mut error = validator::ValidationError::new("length");
                error.message = Some(Cow::from(
                    "LastName length must be between 2 and 512 characters",
                ));
                error.add_param(Cow::from("last_name"), &last_name);
                Err(error)?
            }
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl FromStr for LastName {
    type Err = DomainError;

    fn from_str(last_name: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(last_name.to_owned())
    }
}
