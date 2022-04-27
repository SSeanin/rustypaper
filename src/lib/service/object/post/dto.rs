use crate::domain::post::field::{Content, IsPublished, Shortcode, Title};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostObject {
    pub title: Title,
    pub content: Content,
    pub is_published: IsPublished,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPostObject {
    pub shortcode: Shortcode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostObject {
    pub title: Title,
    pub content: Content,
    pub shortcode: Shortcode,
    pub is_published: IsPublished,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePostObject {
    pub shortcode: Shortcode,
}
