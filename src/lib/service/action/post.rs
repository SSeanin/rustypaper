use crate::data::database::DatabasePool;
use crate::data::query::post::create_post;
use crate::domain::Post;
use crate::service::object::post::CreatePostObject;
use crate::service::Result;

pub async fn create_post_action(
    create_post_object: CreatePostObject,
    database_pool: &DatabasePool,
) -> Result<Post> {
    Ok(create_post(create_post_object, database_pool)
        .await?
        .try_into()?)
}
