use crate::service::ServiceError;
use rocket::serde::json::Json;
use rocket::Responder;

pub mod routes;
pub mod server;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error, Responder)]
pub enum ApiError {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),
}

impl From<ServiceError> for ApiError {
    fn from(service_error: ServiceError) -> Self {
        match service_error {
            ServiceError::NotFound => Self::NotFound(Json("entity not found".to_owned())),
            _ => todo!(),
        }
    }
}
