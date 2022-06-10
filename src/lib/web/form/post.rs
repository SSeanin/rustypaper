use crate::domain::post::field::{Content, IsPublished, Shortcode, Title};
use crate::service::object::post::{CreatePostObject, UpdatePostObject};
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

#[derive(Debug, Deserialize)]
pub struct UpdatePostForm {
    title: Option<String>,
    content: Option<String>,
    is_published: Option<bool>,
    shortcode: String,
}

impl TryFrom<UpdatePostForm> for UpdatePostObject {
    type Error = ApiError;

    fn try_from(form: UpdatePostForm) -> Result<Self, Self::Error> {
        Ok(Self {
            shortcode: form
                .shortcode
                .parse::<Shortcode>()
                .map_err(ServiceError::Domain)?,
            // todo rewrite idiomatic
            title: if let Some(title) = form.title {
                let title = Title::new(title).map_err(ServiceError::Domain)?;
                Some(title)
            } else {
                None
            },

            content: if let Some(content) = form.content {
                let content = Content::new(content).map_err(ServiceError::Domain)?;
                Some(content)
            } else {
                None
            },

            is_published: form.is_published.map(IsPublished::from),
        })
    }
}
