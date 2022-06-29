use crate::domain::Result;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize)]
pub struct FirstName(String);

impl FirstName {
    pub fn new(first_name: String) -> Result<Self> {
        match validator::validate_length(&first_name, Some(2), None, None) {
            true => Ok(Self(first_name)),
            false => {
                let mut error = validator::ValidationError::new("length");
                error.message = Some(Cow::from(
                    "FirstName length must be between 2 and 512 characters",
                ));
                error.add_param(Cow::from("first_name"), &first_name);
                Err(error)?
            }
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
