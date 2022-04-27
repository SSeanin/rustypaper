use crate::data::database::DatabasePool;
use crate::data::id::Id;
use crate::data::model::post::dto::{CreatePostDto, GetPostDto};
use crate::data::model::Post;
use crate::data::Result;
use crate::domain::datetime::AppDatetime;
use sqlx::{query, query_as};

pub async fn get_post<M>(get_post_dto: M, database_pool: &DatabasePool) -> Result<Post>
where
    M: Into<GetPostDto>,
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

pub async fn create_post<M>(create_post_dto: M, database_pool: &DatabasePool) -> Result<Post>
where
    M: Into<CreatePostDto>,
{
    let post = create_post_dto.into();
    let id = Id::new().into_inner();
    let updated_at = AppDatetime::now().into_inner();

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
        id,
        post.title,
        post.content,
        post.shortcode,
        post.is_published,
        updated_at
    )
    .execute(database_pool)
    .await?;

    get_post(post.shortcode, database_pool).await
}
