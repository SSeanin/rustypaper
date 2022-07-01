use crate::domain::user::field::{Email, FirstName, LastName, Password};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserObject {
    pub email: Email,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub password: Password,
}

pub struct GetUserObject {
    pub email: Email,
}

pub struct UpdateUserObject {
    pub email: Email,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub password: Option<Password>,
}
