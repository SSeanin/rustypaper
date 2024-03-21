pub mod field;

use crate::domain::User;
use field::{AuthorId, Content, CreatedAt, IsPublished, PostId, Shortcode, Title, UpdatedAt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(skip)]
    pub post_id: PostId,
    pub title: Title,
    pub content: Content,
    pub shortcode: Shortcode,
    pub is_published: IsPublished,
    #[serde(skip)]
    pub author_id: AuthorId,
    pub author: Option<User>,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}
