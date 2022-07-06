use crate::data::Id;
use crate::domain::user::field::{Email, FirstName, LastName, Password, UserId};
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

pub struct GetUserByIdObject {
    pub id: UserId,
}

impl From<String> for GetUserByIdObject {
    fn from(id: String) -> Self {
        use std::str::FromStr;

        let id = match Id::from_str(id.as_str()) {
            Ok(id) => id,
            Err(..) => Id::nil(),
        };

        Self {
            id: UserId::new(id),
        }
    }
}

pub struct UpdateUserObject {
    pub email: Email,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub password: Option<Password>,
}
