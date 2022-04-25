#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub(self) type Result<T> = std::result::Result<T, DataError>;

pub mod database;
pub mod id;
pub mod model;
pub mod query;
