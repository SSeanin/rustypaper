use crate::{data::DataError, domain::DomainError};
use sqlx::Error::RowNotFound;

pub mod action;
pub mod object;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("data error: {0}")]
    Data(DataError),
    #[error("not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(validator::ValidationErrors),
    #[error("invalid uuid")]
    InvalidUuid,
    #[error("invalid token")]
    InvalidToken,
    #[error("unauthorized")]
    Unauthorized,
}

impl From<DataError> for ServiceError {
    fn from(error: DataError) -> Self {
        match error {
            DataError::Database(e) => match e {
                RowNotFound => Self::NotFound,
                other => Self::Data(DataError::Database(other)),
            },
            DataError::Id(..) => Self::InvalidUuid,
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error {
            RowNotFound => Self::NotFound,
            other => Self::Data(DataError::Database(other)),
        }
    }
}
