use crate::{
    data::Id,
    domain::AppDatetime,
    service::object::user::{CreateUserObject, GetUserByIdObject, GetUserObject, UpdateUserObject},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateUserDto {
    pub(in crate::data) user_id: Uuid,
    pub(in crate::data) first_name: String,
    pub(in crate::data) last_name: String,
    pub(in crate::data) email: String,
    pub(in crate::data) password: String,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

impl From<CreateUserObject> for CreateUserDto {
    fn from(object: CreateUserObject) -> Self {
        Self {
            user_id: Id::new().into_inner(),
            first_name: object.first_name.into_inner(),
            last_name: object.last_name.into_inner(),
            email: object.email.into_inner(),
            password: object.password.into_inner(),
            updated_at: AppDatetime::now().into_inner(),
        }
    }
}

pub struct GetUserByEmailDto {
    pub(in crate::data) email: String,
}

impl From<String> for GetUserByEmailDto {
    fn from(email: String) -> Self {
        Self { email }
    }
}

impl From<GetUserObject> for GetUserByEmailDto {
    fn from(object: GetUserObject) -> Self {
        Self {
            email: object.email.into_inner(),
        }
    }
}

pub struct GetUserByIdDto {
    pub(in crate::data) id: Uuid,
}

impl From<GetUserByIdObject> for GetUserByIdDto {
    fn from(object: GetUserByIdObject) -> Self {
        Self {
            id: object.id.into_inner().into_inner(),
        }
    }
}

pub struct UpdateUserDto {
    pub(in crate::data) email: String,
    pub(in crate::data) first_name: Option<String>,
    pub(in crate::data) last_name: Option<String>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

impl From<UpdateUserObject> for UpdateUserDto {
    fn from(object: UpdateUserObject) -> Self {
        Self {
            first_name: object.first_name.map(|first_name| first_name.into_inner()),
            last_name: object.last_name.map(|last_name| last_name.into_inner()),
            email: object.email.into_inner(),
            password: object.password.map(|password| password.into_inner()),
            updated_at: AppDatetime::now().into_inner(),
        }
    }
}
