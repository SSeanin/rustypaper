use field::{Email, FirstName, LastName, Password, UserId};
use serde::{Deserialize, Serialize};

pub mod field;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip)]
    pub user_id: UserId,
    pub email: Email,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub password: Password,
}
