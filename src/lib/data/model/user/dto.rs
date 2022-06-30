use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateUserDto {
    pub(in crate::data) user_id: Uuid,
    pub(in crate::data) first_name: String,
    pub(in crate::data) last_name: String,
    pub(in crate::data) email: String,
    pub(in crate::data) password: bool,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

pub struct GetUserDto {
    pub(in crate::data) email: String,
}

pub struct UpdateUserDto {
    pub(in crate::data) email: String,
    pub(in crate::data) first_name: Option<String>,
    pub(in crate::data) last_name: Option<String>,
    pub(in crate::data) password: Option<bool>,
    pub(in crate::data) updated_at: DateTime<Utc>,
}
