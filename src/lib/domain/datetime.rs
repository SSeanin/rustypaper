use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum DatetimeError {
    #[error("datetime parse error: {0}")]
    Parse(#[from] ParseError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppDatetime(DateTime<Utc>);

impl AppDatetime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }
}

impl Default for AppDatetime {
    fn default() -> Self {
        Self::now()
    }
}

impl FromStr for AppDatetime {
    type Err = DatetimeError;

    fn from_str(datetime: &str) -> Result<Self, Self::Err> {
        datetime
            .parse::<DateTime<Utc>>()
            .map(Self)
            .map_err(DatetimeError::Parse)
    }
}
