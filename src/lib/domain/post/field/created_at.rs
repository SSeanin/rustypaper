use crate::domain::datetime::AppDatetime;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct CreatedAt(AppDatetime);

impl CreatedAt {
    pub fn into_inner(self) -> AppDatetime {
        self.0
    }
}
