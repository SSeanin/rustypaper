use crate::data::DataError;
use crate::domain::DomainError;
use sqlx::Error::RowNotFound;

pub mod action;
pub mod object;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("data error: {0}")]
    Data(DataError),
    #[error("not found")]
    NotFound,
}

impl From<DataError> for ServiceError {
    fn from(error: DataError) -> Self {
        match error {
            DataError::Database(e) => match e {
                RowNotFound => Self::NotFound,
                other => Self::Data(DataError::Database(other)),
            },
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
