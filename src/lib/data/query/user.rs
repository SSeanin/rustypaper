use crate::data::database::DatabasePool;
use crate::data::model::user::dto::{
    CreateUserDto, GetUserByEmailDto, GetUserByIdDto, UpdateUserDto,
};
use crate::data::model::User;
use crate::data::Result;
use sqlx::{query, query_as};

pub async fn get_user_by_email<D>(get_user_dto: D, database_pool: &DatabasePool) -> Result<User>
where
    D: Into<GetUserByEmailDto>,
{
    let email = get_user_dto.into().email;

    Ok(query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE email = $1
        "#,
        email
    )
    .fetch_one(database_pool)
    .await?)
}

pub async fn get_user_by_id<D>(get_user_by_id_dto: D, database_pool: &DatabasePool) -> Result<User>
where
    D: Into<GetUserByIdDto>,
{
    let id = get_user_by_id_dto.into().id;

    Ok(query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE user_id = $1
        "#,
        id
    )
    .fetch_one(database_pool)
    .await?)
}

pub async fn create_user<D>(create_user_dto: D, database_pool: &DatabasePool) -> Result<User>
where
    D: Into<CreateUserDto>,
{
    let user = create_user_dto.into();

    query!(
        r#"
            INSERT INTO "user" (
                user_id,
                first_name,
                last_name,
                email,
                password,
                updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )
        "#,
        user.user_id,
        user.first_name,
        user.last_name,
        user.email,
        user.password,
        user.updated_at
    )
    .execute(database_pool)
    .await?;

    get_user_by_email(user.email, database_pool).await
}

pub async fn update_user<D>(update_user_dto: D, database_pool: &DatabasePool) -> Result<User>
where
    D: Into<UpdateUserDto>,
{
    let user = update_user_dto.into();

    query!(
        r#"
            UPDATE "user" SET
                first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                password = COALESCE($3, password),
                updated_at = $4
            WHERE email = $5
        "#,
        user.first_name,
        user.last_name,
        user.password,
        user.updated_at,
        user.email
    )
    .execute(database_pool)
    .await?;

    get_user_by_email(user.email, database_pool).await
}
