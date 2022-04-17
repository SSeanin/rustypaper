use serde::{Serialize, Deserialize};
use derive_more::{Constructor, From};
use crate::data::id::Id;

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct PostId(Id);

impl PostId {
    pub fn into_inner(self) -> Id {
        self.0
    }
}

impl From<Id> for PostId {
    fn from(id: Id) -> Self {
        Self(id)
    }
}

impl Default for PostId {
    fn default() -> Self {
        Self(Id::nil())
    }
}
