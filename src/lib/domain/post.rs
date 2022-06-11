pub mod field;

use field::{Content, CreatedAt, IsPublished, PostId, Shortcode, Title, UpdatedAt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(skip)]
    pub post_id: PostId,
    pub title: Title,
    pub content: Content,
    pub shortcode: Shortcode,
    pub is_published: IsPublished,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}
