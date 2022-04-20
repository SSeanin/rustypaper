use std::str::FromStr;
use serde::{Serialize, Deserialize};
use super::FieldError;

#[derive(Debug, Serialize, Deserialize)]
pub struct IsPublished(bool);

impl IsPublished {
    pub fn into_inner(self) -> bool {
        self.0
    }
}

impl Default for IsPublished {
    fn default() -> Self {
        Self(true)
    }
}

impl FromStr for IsPublished {
    type Err = FieldError;

    fn from_str(boolean: &str) -> Result<Self, Self::Err> {
        str::parse::<bool>(boolean)
            .map(Self)
            .map_err(FieldError::ParseBool)
    }
}
