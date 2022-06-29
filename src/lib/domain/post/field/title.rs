use crate::domain::{DomainError, Result};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    pub fn new<T>(title: T) -> Result<Self>
    where
        T: Into<String>,
    {
        let title = title.into();
        match validator::validate_length(&title, Some(2), Some(512), None) {
            true => Ok(Self(title)),
            false => {
                let mut error = validator::ValidationError::new("length");
                error.message = Some(Cow::from(
                    "Title length must be between 2 and 512 characters",
                ));
                error.add_param(Cow::from("title"), &title);
                Err(error)?
            }
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Title {
    type Err = DomainError;

    fn from_str(title: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(title)
    }
}
