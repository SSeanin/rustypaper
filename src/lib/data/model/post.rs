pub mod dto;

use sqlx::types::{time::OffsetDateTime, Uuid};

#[derive(Debug)]
pub struct Post {
    pub(self) post_id: Uuid,
    pub(self) title: String,
    pub(self) content: String,
    pub(self) shortcode: String,
    pub(self) is_published: bool,
    pub(self) created_at: OffsetDateTime,
    pub(self) updated_at: OffsetDateTime,
}
