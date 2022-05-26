use crate::data::database::DatabasePool;
use crate::data::query::post::{create_post, delete_post, get_all_posts, get_post, update_post};
use crate::domain::Post;
use crate::service::object::post::{
    CreatePostObject, DeletePostObject, GetAllPostsObject, GetPostObject, UpdatePostObject,
};
use crate::service::Result;

pub async fn create_post_action(
    create_post_object: CreatePostObject,
    database_pool: &DatabasePool,
) -> Result<Post> {
    Ok(create_post(create_post_object, database_pool)
        .await?
        .try_into()?)
}

pub async fn get_all_posts_action<O>(
    get_all_posts_object: O,
    database_pool: &DatabasePool,
) -> Result<Vec<Post>>
where
    O: Into<GetAllPostsObject>,
{
    let posts = get_all_posts(get_all_posts_object.into(), database_pool)
        .await?
        .into_iter()
        .map(|post| {
            post.try_into()
                .expect("conversion of crate::data::model::Post to crate::domain::Post failed")
        })
        .collect::<Vec<Post>>();

    Ok(posts)
}

pub async fn get_post_action<O>(get_post_object: O, database_pool: &DatabasePool) -> Result<Post>
where
    O: Into<GetPostObject>,
{
    Ok(get_post(get_post_object.into(), database_pool)
        .await?
        .try_into()?)
}

pub async fn update_post_action<O>(
    update_post_object: O,
    database_pool: &DatabasePool,
) -> Result<Post>
where
    O: Into<UpdatePostObject>,
{
    Ok(update_post(update_post_object.into(), database_pool)
        .await?
        .try_into()?)
}

pub async fn delete_post_action<O>(
    delete_post_object: O,
    database_pool: &DatabasePool,
) -> Result<()>
where
    O: Into<DeletePostObject>,
{
    Ok(delete_post(delete_post_object.into(), database_pool).await?)
}
