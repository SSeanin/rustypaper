use crate::data::database::DatabasePool;
use crate::data::query::user::get_user_by_email;
use crate::domain::{TokenGenerator, TokenPair, User};
use crate::service::object::auth::LoginObject;
use crate::service::{Result, ServiceError};

pub async fn login_action<O>(
    login_object: O,
    database_pool: &DatabasePool,
    token_generator: &TokenGenerator,
) -> Result<TokenPair>
where
    O: Into<LoginObject>,
{
    let object = login_object.into();

    let user: User = get_user_by_email(object.email, database_pool)
        .await?
        .try_into()?;

    match user.password.verify(object.password.as_str()) {
        Ok(..) => Ok(token_generator.generate_token_pair(user.user_id.to_string().as_str())?),
        Err(e) => Err(ServiceError::Domain(e)),
    }
}
