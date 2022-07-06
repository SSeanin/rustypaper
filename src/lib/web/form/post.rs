use crate::domain::post::field::{AuthorId, Content, IsPublished, Shortcode, Title};
use crate::domain::user::field::UserId;
use crate::domain::DomainError;
use crate::service::object::post::{CreatePostObject, UpdatePostObject};
use crate::service::ServiceError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePostForm {
    pub(in crate::web) title: String,
    pub(in crate::web) content: String,
    pub(in crate::web) is_published: bool,
    #[serde(skip)]
    pub(in crate::web) author_id: UserId,
}

impl TryFrom<CreatePostForm> for CreatePostObject {
    type Error = ServiceError;

    fn try_from(form: CreatePostForm) -> Result<Self, Self::Error> {
        let mut validation_errors = validator::ValidationErrors::new();

        let title = Title::new(form.title);
        let content = Content::new(form.content);

        if let Err(DomainError::Validation(validation_error)) = &title {
            validation_errors.add("title", validation_error.clone());
        }
        if let Err(DomainError::Validation(validation_error)) = &content {
            validation_errors.add("content", validation_error.clone());
        }

        if validation_errors.is_empty() {
            Ok(Self {
                title: title.expect("failed to parse title"),
                content: content.expect("failed to parse content"),
                is_published: IsPublished::from(form.is_published),
                author_id: AuthorId::new(form.author_id),
            })
        } else {
            Err(ServiceError::Validation(validation_errors))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostForm {
    pub(in crate::web) title: Option<String>,
    pub(in crate::web) content: Option<String>,
    pub(in crate::web) is_published: Option<bool>,
    #[serde(skip)]
    pub(in crate::web) shortcode: Option<String>,
}

impl TryFrom<UpdatePostForm> for UpdatePostObject {
    type Error = ServiceError;

    fn try_from(form: UpdatePostForm) -> Result<Self, Self::Error> {
        let mut validation_errors = validator::ValidationErrors::new();

        let title = form.title.map(Title::new);
        let content = form.content.map(Content::new);

        if let Some(Err(DomainError::Validation(validation_error))) = &title {
            validation_errors.add("title", validation_error.clone());
        }

        if let Some(Err(DomainError::Validation(validation_error))) = &content {
            validation_errors.add("content", validation_error.clone());
        }

        if validation_errors.is_empty() {
            Ok(Self {
                shortcode: form
                    .shortcode
                    .unwrap_or_default()
                    .parse::<Shortcode>()
                    .map_err(ServiceError::Domain)?,
                title: title.map(|title| title.expect("failed to parse title")),
                content: content.map(|content| content.expect("failed to parse content")),
                is_published: form.is_published.map(IsPublished::from),
            })
        } else {
            Err(ServiceError::Validation(validation_errors))
        }
    }
}
