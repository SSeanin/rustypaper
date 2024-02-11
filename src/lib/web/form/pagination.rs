use crate::service::object::post::GetAllPostsObject;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationForm {
    limit: Option<i64>,
    skip: Option<i64>,
}

impl Default for PaginationForm {
    fn default() -> Self {
        Self {
            limit: Some(8),
            skip: Some(0),
        }
    }
}

impl From<PaginationForm> for GetAllPostsObject {
    fn from(form: PaginationForm) -> Self {
        Self {
            limit: form.limit,
            skip: form.skip,
        }
    }
}
