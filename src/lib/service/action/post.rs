use crate::data::database::DatabasePool;
use crate::data::query::post::{create_post, get_post, update_post};
use crate::domain::Post;
use crate::service::object::post::{CreatePostObject, GetPostObject, UpdatePostObject};
use crate::service::Result;

pub async fn create_post_action(
    create_post_object: CreatePostObject,
    database_pool: &DatabasePool,
) -> Result<Post> {
    Ok(create_post(create_post_object, database_pool)
        .await?
        .try_into()?)
}

pub async fn get_post_action(
    get_post_object: GetPostObject,
    database_pool: &DatabasePool,
) -> Result<Post> {
    Ok(get_post(get_post_object, database_pool).await?.try_into()?)
}

pub async fn update_post_action(
    update_post_object: UpdatePostObject,
    database_pool: &DatabasePool,
) -> Result<Post> {
    Ok(update_post(update_post_object, database_pool)
        .await?
        .try_into()?)
}
