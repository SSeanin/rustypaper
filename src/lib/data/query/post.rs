use crate::data::{
    database::DatabasePool,
    model::{
        post::dto::{CreatePostDto, DeletePostDto, GetAllPostsDto, GetPostDto, UpdatePostDto},
        Post,
    },
    Result,
};
use sqlx::{query, query_as};

pub async fn get_all_posts<D>(
    get_all_posts_dto: D,
    database_pool: &DatabasePool,
) -> Result<Vec<Post>>
where
    D: Into<GetAllPostsDto>,
{
    let get_all_posts_dto = get_all_posts_dto.into();

    Ok(query_as!(
        Post,
        r#"
            SELECT
                post_id,
                title,
                LEFT(content, 80) as content,
                shortcode,
                author_id,
                is_published,
                post.created_at,
                post.updated_at,
                user_id,
                first_name as user_first_name,
                last_name as user_last_name
            FROM post INNER JOIN "user" ON post.author_id = "user".user_id ORDER BY post.created_at DESC LIMIT $1 OFFSET $2
        "#,
        get_all_posts_dto.limit,
        get_all_posts_dto.skip
    )
    .fetch_all(database_pool)
    .await?)
}

pub async fn get_post<D>(get_post_dto: D, database_pool: &DatabasePool) -> Result<Post>
where
    D: Into<GetPostDto>,
{
    let shortcode = get_post_dto.into().shortcode;
    Ok(query_as!(
        Post,
        r#"
            SELECT
                post_id,
                title,
                content,
                shortcode,
                author_id,
                is_published,
                post.created_at,
                post.updated_at,
                user_id,
                first_name as user_first_name,
                last_name as user_last_name
            FROM post JOIN "user" ON post.author_id = "user".user_id AND shortcode = $1;
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
                author_id,
                updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
        "#,
        post.post_id,
        post.title,
        post.content,
        post.shortcode,
        post.is_published,
        post.author_id,
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

    query!(
        r#"
            UPDATE post SET
                title = COALESCE($1, title),
                content = COALESCE($2, content),
                is_published = COALESCE($3, is_published),
                updated_at = $4
            WHERE shortcode = $5
        "#,
        post.title,
        post.content,
        post.is_published,
        post.updated_at,
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
