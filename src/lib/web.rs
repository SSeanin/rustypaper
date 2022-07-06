use crate::domain::DomainError;
use crate::service::ServiceError;
use crate::web::response::{ErrorResponse, FailResponse};
use rocket::serde::json::Json;
use rocket::Responder;
use validator::ValidationErrors;

pub mod catcher;
pub mod form;
pub mod response;
pub mod router;
pub mod server;

pub use server::rocket;
pub use server::RocketConfig;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error, Responder)]
pub enum ApiError {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<FailResponse<String>>),

    #[error("internal sever error")]
    #[response(status = 500, content_type = "json")]
    Internal(Json<ErrorResponse<String>>),

    #[error("validation error")]
    #[response(status = 400, content_type = "json")]
    Validation(Json<FailResponse<ValidationErrors>>),

    #[error("unauthorized")]
    #[response(status = 401, content_type = "json")]
    Unauthorized(Json<FailResponse<String>>),
}

impl From<ServiceError> for ApiError {
    fn from(service_error: ServiceError) -> Self {
        match service_error {
            ServiceError::NotFound => Self::NotFound(Json(FailResponse::new(
                "requested entity was now found on this server".to_owned(),
            ))),

            ServiceError::Validation(validation_errors) => {
                Self::Validation(Json(FailResponse::new(validation_errors)))
            }

            ServiceError::Unauthorized => {
                Self::Unauthorized(Json(FailResponse::new("unauthorized".to_owned())))
            }

            ServiceError::Domain(domain_error) => match domain_error {
                DomainError::InvalidPassword | DomainError::Token(..) => {
                    Self::Unauthorized(Json(FailResponse::new("unauthorized".to_owned())))
                }
                _ => Self::Internal(Json(ErrorResponse::new(
                    "internal server error".to_owned(),
                    None,
                ))),
            },

            _ => Self::Internal(Json(ErrorResponse::new(
                "internal server error".to_owned(),
                None,
            ))),
        }
    }
}
