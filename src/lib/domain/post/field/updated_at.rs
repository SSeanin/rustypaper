use serde::{Serialize, Deserialize};
use derive_more::Constructor;
use crate::domain::datetime::Datetime;

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct UpdatedAt(Datetime);

impl UpdatedAt {
    pub fn into_inner(self) -> Datetime {
        self.0
    }
}
