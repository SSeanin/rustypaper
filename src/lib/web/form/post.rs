use crate::domain::post::field::{Content, IsPublished, Title};
use crate::service::object::post::CreatePostObject;
use crate::service::ServiceError;
use crate::web::ApiError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePostForm {
    title: String,
    content: String,
    is_published: bool,
}

impl TryFrom<CreatePostForm> for CreatePostObject {
    type Error = ApiError;

    fn try_from(form: CreatePostForm) -> Result<Self, Self::Error> {
        Ok(Self {
            title: Title::new(form.title).map_err(ServiceError::Domain)?,
            content: Content::new(form.content).map_err(ServiceError::Domain)?,
            is_published: IsPublished::from(form.is_published),
        })
    }
}
