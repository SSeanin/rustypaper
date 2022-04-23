use crate::domain::validation::ValidationError;

pub mod content;
pub mod created_at;
pub mod is_published;
pub mod post_id;
pub mod shortcode;
pub mod title;
pub mod updated_at;

pub use content::Content;
pub use created_at::CreatedAt;
pub use is_published::IsPublished;
pub use post_id::PostId;
pub use shortcode::Shortcode;
pub use title::Title;
pub use updated_at::UpdatedAt;

pub(self) type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, thiserror::Error)]
pub enum FieldError {
    #[error("field validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("invalid boolean value: {0}")]
    ParseBool(#[from] std::str::ParseBoolError),
}
