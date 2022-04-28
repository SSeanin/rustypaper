pub mod dto;

use crate::data::Id;
use crate::domain;
use crate::domain::post::field::{
    Content, CreatedAt, IsPublished, PostId, Shortcode, Title, UpdatedAt,
};
use sqlx::types::{chrono::DateTime, chrono::Utc, Uuid};

#[derive(Debug)]
pub struct Post {
    pub(in crate::data) post_id: Uuid,
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) is_published: bool,
    pub(in crate::data) created_at: DateTime<Utc>,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

impl TryFrom<Post> for domain::Post {
    type Error = domain::DomainError;

    fn try_from(raw: Post) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        Ok(Self {
            post_id: PostId::from(Id::from(raw.post_id)),
            title: Title::from_str(raw.title.as_str())?,
            content: Content::from_str(raw.content.as_str())?,
            shortcode: Shortcode::from_str(raw.shortcode.as_str())?,
            is_published: raw.is_published.into(),
            created_at: CreatedAt::new(raw.created_at.into()),
            updated_at: UpdatedAt::new(raw.updated_at.into()),
        })
    }
}
