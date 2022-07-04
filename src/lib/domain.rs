pub mod datetime;
pub mod post;
pub mod timestamp;
pub mod token_generator;
pub mod user;
pub mod validation;

pub use datetime::AppDatetime;
pub use post::Post;
pub use token_generator::{TokenGenerator, TokenPair};
pub use user::User;

pub(self) type Result<T> = std::result::Result<T, DomainError>;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("field validation failed: {0}")]
    Validation(#[from] validator::ValidationError),
    #[error("invalid boolean value: {0}")]
    ParseBool(#[from] std::str::ParseBoolError),
    #[error("password hashing failed: {0}")]
    Password(argon2::password_hash::Error),
    #[error("token error: {0}")]
    Token(#[from] jsonwebtoken::errors::Error),
}

impl From<argon2::password_hash::Error> for DomainError {
    fn from(password_hash_error: argon2::password_hash::Error) -> Self {
        // todo match for password errors
        Self::Password(password_hash_error)
    }
}
