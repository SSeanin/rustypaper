use crate::data::database::DatabasePool;
use crate::data::query::user::{create_user, get_user_by_id};
use crate::domain::User;
use crate::service::object::user::dto::GetUserByIdObject;
use crate::service::object::user::CreateUserObject;
use crate::service::Result;

pub async fn get_user_by_id_action<O>(
    get_user_by_id_object: O,
    database_pool: &DatabasePool,
) -> Result<User>
where
    O: Into<GetUserByIdObject>,
{
    Ok(get_user_by_id(get_user_by_id_object.into(), database_pool)
        .await?
        .try_into()?)
}

pub async fn create_user_action<O>(
    create_user_object: O,
    database_pool: &DatabasePool,
) -> Result<User>
where
    O: Into<CreateUserObject>,
{
    Ok(create_user(create_user_object.into(), database_pool)
        .await?
        .try_into()?)
}
