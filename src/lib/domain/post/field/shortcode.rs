use crate::domain::DomainError;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Shortcode(String);

impl Shortcode {
    pub fn new() -> Self {
        let shortcode = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>();

        Self(shortcode)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for Shortcode {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Shortcode {
    type Err = DomainError;

    fn from_str(shortcode: &str) -> Result<Self, Self::Err> {
        Ok(Self(shortcode.into()))
    }
}
