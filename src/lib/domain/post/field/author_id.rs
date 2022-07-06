use crate::domain::user::field::UserId;
use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, Constructor, From)]
pub struct AuthorId(UserId);

impl AuthorId {
    pub fn into_inner(self) -> UserId {
        self.0
    }
}

impl fmt::Display for AuthorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.into_inner().to_string())
    }
}
