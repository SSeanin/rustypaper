pub mod dto;

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
