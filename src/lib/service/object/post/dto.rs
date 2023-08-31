use crate::domain::post::field::{AuthorId, Content, IsPublished, Shortcode, Title};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostObject {
    pub title: Title,
    pub content: Content,
    pub is_published: IsPublished,
    pub author_id: AuthorId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllPostsObject {
    pub skip: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPostObject {
    pub shortcode: Shortcode,
}

impl From<&str> for GetPostObject {
    fn from(shortcode: &str) -> Self {
        Self {
            shortcode: shortcode.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostObject {
    pub title: Option<Title>,
    pub content: Option<Content>,
    pub shortcode: Shortcode,
    pub is_published: Option<IsPublished>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePostObject {
    pub shortcode: Shortcode,
}

impl From<&str> for DeletePostObject {
    fn from(shortcode: &str) -> Self {
        Self {
            shortcode: shortcode.into(),
        }
    }
}
