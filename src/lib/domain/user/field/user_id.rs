use crate::data::id::Id;
use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, Constructor, From, Copy, Clone, Eq, PartialEq)]
pub struct UserId(Id);

impl UserId {
    pub fn into_inner(self) -> Id {
        self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Id::nil())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.into_inner().to_string())
    }
}
