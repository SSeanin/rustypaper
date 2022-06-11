use crate::data::id::Id;
use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor, From)]
pub struct UserId(Id);

impl UserId {
    pub fn into_inner(self) -> Id {
        self.0
    }
}
