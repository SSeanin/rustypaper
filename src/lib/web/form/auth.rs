use crate::{
    domain::{
        token_generator::{AccessTokenClaims, RefreshTokenClaims},
        user::field::{Email, FirstName, LastName, Password},
        DomainError,
    },
    service::{
        object::{
            auth::{LoginObject, RefreshObject},
            user::CreateUserObject,
        },
        ServiceError,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignupForm {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl TryFrom<SignupForm> for CreateUserObject {
    type Error = ServiceError;

    fn try_from(form: SignupForm) -> Result<Self, Self::Error> {
        let mut validation_errors = validator::ValidationErrors::new();

        let first_name = FirstName::new(form.first_name);
        let last_name = LastName::new(form.last_name);
        let email = Email::new(form.email);
        let password = Password::new(form.password);

        if let Err(DomainError::Validation(validation_error)) = &first_name {
            validation_errors.add("first_name", validation_error.clone())
        }
        if let Err(DomainError::Validation(validation_error)) = &last_name {
            validation_errors.add("last_name", validation_error.clone())
        }
        if let Err(DomainError::Validation(validation_error)) = &email {
            validation_errors.add("email", validation_error.clone())
        }
        if let Err(DomainError::Validation(validation_error)) = &password {
            validation_errors.add("password", validation_error.clone())
        }

        if validation_errors.is_empty() {
            Ok(Self {
                first_name: first_name.expect("failed to parse first_name"),
                last_name: last_name.expect("failed to parse last_name"),
                email: email.expect("failed to parse email"),
                password: password.expect("failed to parse password"),
            })
        } else {
            Err(ServiceError::Validation(validation_errors))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

impl From<LoginForm> for LoginObject {
    fn from(form: LoginForm) -> Self {
        Self {
            email: form.email,
            password: form.password,
        }
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub refresh_token: String,
}

pub struct RefreshForm {
    pub access_token_claims: AccessTokenClaims,
    pub refresh_token_claims: RefreshTokenClaims,
}

impl From<RefreshForm> for RefreshObject {
    fn from(form: RefreshForm) -> Self {
        Self {
            access_token_claims: form.access_token_claims,
            refresh_token_claims: form.refresh_token_claims,
        }
    }
}
