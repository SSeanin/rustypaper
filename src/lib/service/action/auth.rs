use crate::{
    data::{database::DatabasePool, query::user::get_user_by_email},
    domain::{TokenGenerator, TokenPair, User},
    service::{
        object::auth::{LoginObject, RefreshObject},
        Result, ServiceError,
    },
};

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

pub async fn refresh_action<O>(
    refresh_object: O,
    _database_pool: &DatabasePool,
    token_generator: &TokenGenerator,
) -> Result<TokenPair>
where
    O: Into<RefreshObject>,
{
    let object = refresh_object.into();

    if object.refresh_token_claims.sid == object.access_token_claims.jti {
        // todo persist refresh token family
        Ok(token_generator.generate_token_pair(object.access_token_claims.sub.as_str())?)
    } else {
        Err(ServiceError::InvalidToken)
    }
}
