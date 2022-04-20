use std::str::FromStr;
use serde::{Serialize, Deserialize};
use time::{OffsetDateTime, format_description, error::Parse};

#[derive(Debug, thiserror::Error)]
pub enum DatetimeError {
    #[error("datetime parse error: {0}")]
    Parse(#[from] Parse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datetime(OffsetDateTime);

impl Datetime {
    pub fn into_inner(self) -> OffsetDateTime {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }
}

impl Default for Datetime {
    fn default() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl FromStr for Datetime {
    type Err = DatetimeError;

    fn from_str(datetime: &str) -> Result<Self, Self::Err> {
        let format = format_description::well_known::Rfc3339;
        OffsetDateTime::parse(datetime, &format)
            .map(Self)
            .map_err(DatetimeError::Parse)
    }
}
