use serde::{Serialize, Deserialize};
use derive_more::{Constructor, From};
use crate::data::id::Id;

#[derive(Debug, Serialize, Deserialize, Constructor, From)]
pub struct PostId(Id);

impl PostId {
    pub fn into_inner(self) -> Id {
        self.0
    }
}

impl Default for PostId {
    fn default() -> Self {
        Self(Id::nil())
    }
}
