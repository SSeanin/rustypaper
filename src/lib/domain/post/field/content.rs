use crate::domain::{DomainError, Result};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn new<T>(content: T) -> Result<Self>
    where
        T: Into<String>,
    {
        let content = content.into();

        match validator::validate_length(&content, Some(8), None, None) {
            true => Ok(Self(content)),
            false => {
                let mut error = validator::ValidationError::new("length");
                error.message = Some(Cow::from("Content length must be longer than 8 characters"));
                error.add_param(Cow::from("content"), &content);
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

impl FromStr for Content {
    type Err = DomainError;

    fn from_str(content: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(content)
    }
}
