use crate::service::object::post::GetAllPostsObject;
use rocket::FromForm;

#[derive(Debug, FromForm)]
pub struct PaginationForm {
    limit: Option<i64>,
    skip: Option<i64>,
}

impl From<PaginationForm> for GetAllPostsObject {
    fn from(form: PaginationForm) -> Self {
        Self {
            limit: form.limit,
            skip: form.skip,
        }
    }
}
