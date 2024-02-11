pub mod dto;

use crate::{
    data::Id,
    domain::user::field::{CreatedAt, Email, FirstName, LastName, Password, UpdatedAt, UserId},
    domain::DomainError,
};
use sqlx::types::{chrono::DateTime, chrono::Utc, Uuid};

pub struct User {
    pub(in crate::data) user_id: Uuid,
    pub(in crate::data) first_name: String,
    pub(in crate::data) last_name: String,
    pub(in crate::data) email: String,
    pub(in crate::data) password: String,
    pub(in crate::data) created_at: DateTime<Utc>,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

impl TryFrom<User> for crate::domain::User {
    type Error = DomainError;

    fn try_from(raw: User) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        Ok(Self {
            user_id: UserId::from(Id::from(raw.user_id)),
            email: Email::from_str(raw.email.as_str())?,
            first_name: FirstName::from_str(raw.first_name.as_str())?,
            last_name: LastName::from_str(raw.last_name.as_str())?,
            password: Password::from(raw.password),
            created_at: CreatedAt::new(raw.created_at.into()),
            updated_at: UpdatedAt::new(raw.updated_at.into()),
        })
    }
}
