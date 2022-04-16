use std::str::FromStr;
use serde::{Serialize, Deserialize};
use super::{FieldError, Result};
use crate::domain::validation::string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn new<T>(content: T) -> Result<Self> where T: Into<String> {
        let content = string::Check::new(content)
            .is_min_length(8)?
            .is_not_empty()?
            .into_inner();

        Ok(Self(content))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Content {
    type Err = FieldError;

    fn from_str(content: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(content)
    }
}
