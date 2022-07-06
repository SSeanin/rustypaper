pub mod database;
pub mod id;
pub mod model;
pub mod query;

pub use id::Id;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("id error: {0}")]
    Id(#[from] id::IdError),
}

pub(self) type Result<T> = std::result::Result<T, DataError>;
