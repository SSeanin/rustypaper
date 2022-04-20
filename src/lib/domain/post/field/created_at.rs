use serde::{Serialize, Deserialize};
use derive_more::Constructor;
use crate::domain::datetime::Datetime;

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct CreatedAt(Datetime);

impl CreatedAt {
    pub fn into_inner(self) -> Datetime {
        self.0
    }
}
