use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("uuid error: {0}")]
    Uuid(#[from] uuid::Error),
}

#[derive(Debug, Serialize, Deserialize, From, Display, Copy, Clone)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Uuid::nil().into()
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(id)?))
    }
}
