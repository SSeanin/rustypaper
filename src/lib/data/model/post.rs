pub mod dto;

use crate::{
    data::Id,
    domain::{
        self,
        post::field::{
            AuthorId, Content, CreatedAt, IsPublished, PostId, Shortcode, Title, UpdatedAt,
        },
        user::field::{Email, FirstName, LastName, Password, UserId},
        User,
    },
};
use sqlx::types::{chrono::DateTime, chrono::Utc, Uuid};

#[derive(Debug)]
pub struct Post {
    pub(in crate::data) post_id: Uuid,
    pub(in crate::data) title: String,
    pub(in crate::data) content: Option<String>,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) is_published: bool,
    pub(in crate::data) author_id: Uuid,
    pub(in crate::data) created_at: DateTime<Utc>,
    pub(in crate::data) updated_at: DateTime<Utc>,
    pub(in crate::data) user_id: Uuid,
    pub(in crate::data) user_first_name: String,
    pub(in crate::data) user_last_name: String,
}

impl TryFrom<Post> for domain::Post {
    type Error = domain::DomainError;

    fn try_from(raw: Post) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        Ok(Self {
            post_id: PostId::from(Id::from(raw.post_id)),
            title: Title::from_str(raw.title.as_str())?,
            content: Content::from_str(raw.content.unwrap_or_default().as_str())?,
            shortcode: Shortcode::from_str(raw.shortcode.as_str())?,
            is_published: IsPublished::from(raw.is_published),
            author_id: AuthorId::from(UserId::from(Id::from(raw.author_id))),
            created_at: CreatedAt::new(raw.created_at.into()),
            updated_at: UpdatedAt::new(raw.updated_at.into()),
            author: Some(User {
                user_id: UserId::from(Id::from(raw.user_id)),
                first_name: FirstName::from_str(raw.user_first_name.as_str())?,
                last_name: LastName::from_str(raw.user_last_name.as_str())?,
                email: Email::default(),
                password: Password::default(),
                created_at: CreatedAt::default(),
                updated_at: UpdatedAt::default(),
            }),
        })
    }
}
