pub mod string;

pub(self) type Result<T> = std::result::Result<T, ValidationError>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ValidationError {
    #[error("value too short: min {min} characters, got {got} characters")]
    MinLength { min: usize, got: usize },
    #[error("value too short: max {max} characters, got {got} characters")]
    MaxLength { max: usize, got: usize },
    #[error("invalid characters")]
    InvalidCharacter,
    #[error("empty content")]
    EmptyContent,
}
