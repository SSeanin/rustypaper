use crate::domain::DomainError;
use derive_more::From;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::request::FromParam;
use rocket::{UriDisplayPath, UriDisplayQuery};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, UriDisplayPath, UriDisplayQuery, From)]
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

impl From<&str> for Shortcode {
    fn from(shortcode: &str) -> Self {
        Self(shortcode.to_owned())
    }
}

impl FromStr for Shortcode {
    type Err = DomainError;

    fn from_str(shortcode: &str) -> Result<Self, Self::Err> {
        Ok(Self(shortcode.into()))
    }
}

impl<'r> FromParam<'r> for Shortcode {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Self::from(param))
    }
}
