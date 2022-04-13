use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::domain::validation::{string, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    // todo maybe a custom error enum for fields
    pub fn new<T>(title: T) -> Result<Self, ValidationError> where T: Into<String> {
        let title = string::Check::new(title)
            .is_min_length(2)?
            .is_max_length(256)?
            .is_not_empty()?
            .inner_value();

        Ok(Self(title))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl FromStr for Title {
    type Err = ValidationError;

    fn from_str(title: &str) -> Result<Self, Self::Err> {
        Self::new(title)
    }
}
