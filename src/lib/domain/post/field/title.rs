use std::str::FromStr;
use serde::{Serialize, Deserialize};
use super::{FieldError, Result};
use crate::domain::validation::{string};

#[derive(Debug, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    pub fn new<T>(title: T) -> Result<Self> where T: Into<String> {
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
}

impl FromStr for Title {
    type Err = FieldError;

    fn from_str(title: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(title)
    }
}
