use field::{CreatedAt, Email, FirstName, LastName, Password, UpdatedAt, UserId};
use serde::{Deserialize, Serialize};

pub mod field;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip)]
    pub user_id: UserId,
    pub email: Email,
    pub first_name: FirstName,
    pub last_name: LastName,
    #[serde(skip)]
    pub password: Password,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}
