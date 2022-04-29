use crate::data::database::DatabasePool;
use crate::data::model::post::dto::{CreatePostDto, DeletePostDto, GetPostDto, UpdatePostDto};
use crate::data::model::Post;
use crate::data::Result;
use crate::domain::datetime::AppDatetime;
use sqlx::{query, query_as};

pub async fn get_post<D>(get_post_dto: D, database_pool: &DatabasePool) -> Result<Post>
where
    D: Into<GetPostDto>,
{
    let shortcode = get_post_dto.into().shortcode;
    Ok(query_as!(
        Post,
        r#"
            SELECT * FROM post WHERE shortcode = $1
        "#,
        shortcode
    )
    .fetch_one(database_pool)
    .await?)
}

pub async fn create_post<D>(create_post_dto: D, database_pool: &DatabasePool) -> Result<Post>
where
    D: Into<CreatePostDto>,
{
    let post = create_post_dto.into();

    query!(
        r#"
            INSERT INTO post (
                post_id,
                title,
                content,
                shortcode,
                is_published,
                updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )
        "#,
        post.post_id,
        post.title,
        post.content,
        post.shortcode,
        post.is_published,
        post.updated_at
    )
    .execute(database_pool)
    .await?;

    get_post(post.shortcode, database_pool).await
}

pub async fn update_post<D>(update_post_dto: D, database_pool: &DatabasePool) -> Result<Post>
where
    D: Into<UpdatePostDto>,
{
    let post = update_post_dto.into();
    let updated_at = AppDatetime::now().into_inner();

    query!(
        r#"
            UPDATE post SET
                title = $1,
                content = $2,
                is_published = $3,
                updated_at = $4
            WHERE shortcode = $5
        "#,
        post.title,
        post.content,
        post.is_published,
        updated_at,
        post.shortcode
    )
    .execute(database_pool)
    .await?;

    get_post(post.shortcode, database_pool).await
}

pub async fn delete_post<D>(delete_post_dto: D, database_pool: &DatabasePool) -> Result<()>
where
    D: Into<DeletePostDto>,
{
    let shortcode = delete_post_dto.into().shortcode;

    query!(
        r#"
            DELETE FROM post WHERE shortcode = $1
        "#,
        shortcode
    )
    .execute(database_pool)
    .await?;

    Ok(())
}
