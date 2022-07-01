use crate::data::database::DatabasePool;
use crate::data::query::user::create_user;
use crate::domain::User;
use crate::service::object::user::CreateUserObject;
use crate::service::Result;

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
