use crate::service::ServiceError;
use crate::web::response::FailResponse;
use rocket::serde::json::Json;
use rocket::Responder;

pub mod catcher;
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
    Internal(Json<String>),
}

impl From<ServiceError> for ApiError {
    fn from(service_error: ServiceError) -> Self {
        match service_error {
            ServiceError::NotFound => Self::NotFound(Json(FailResponse::new(
                "requested entity was now found on this server".to_owned(),
            ))),
            ServiceError::Data(..) => Self::Internal(Json("internal server error".to_owned())),
            // todo handle domain error types here
            ServiceError::Domain(..) => Self::Internal(Json("internal server error".to_owned())),
        }
    }
}
