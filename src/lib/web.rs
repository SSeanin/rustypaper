use crate::{
    domain::DomainError,
    service::ServiceError,
    web::response::{ErrorResponse, FailResponse},
};
use axum::{http::status::StatusCode, Json};

pub mod extractor;
pub mod form;
pub mod response;
pub mod router;
pub mod server;

pub use server::Config;

pub type Result<T> = axum::response::Result<T>;

impl axum::response::IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServiceError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(FailResponse::new(
                    "requested entity was not found on this server".to_owned(),
                )),
            )
                .into_response(),

            ServiceError::Validation(validation_errors) => (
                StatusCode::BAD_REQUEST,
                Json(FailResponse::new(validation_errors)),
            )
                .into_response(),

            ServiceError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(FailResponse::new("unauthorized".to_owned())),
            )
                .into_response(),

            ServiceError::Domain(domain_error) => match domain_error {
                DomainError::InvalidPassword | DomainError::Token(..) => (
                    StatusCode::UNAUTHORIZED,
                    Json(FailResponse::new("unauthorized".to_owned())),
                )
                    .into_response(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::<String>::new(
                        "internal server error".to_owned(),
                        None,
                    )),
                )
                    .into_response(),
            },

            ServiceError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(FailResponse::new("unauthorized".to_owned())),
            )
                .into_response(),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::<String>::new(
                    "internal server error".to_owned(),
                    None,
                )),
            )
                .into_response(),
        }
    }
}
