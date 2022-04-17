use crate::domain::validation::ValidationError;

pub mod post_id;
pub mod title;
pub mod content;
pub mod shortcode;
pub mod published;
pub mod created_at;
pub mod updated_at;

pub(self) type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, thiserror::Error)]
pub enum FieldError {
    #[error("field validation failed: {0}")]
    Validation(#[from] ValidationError)
}
