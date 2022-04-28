use crate::domain::validation::string;
use crate::domain::{DomainError, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    pub fn new<T>(title: T) -> Result<Self>
    where
        T: Into<String>,
    {
        let title = string::Check::new(title)
            .is_min_length(2)?
            .is_max_length(256)?
            .is_not_empty()?
            .into_inner();

        Ok(Self(title))
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
