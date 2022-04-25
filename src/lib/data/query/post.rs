use crate::data::database::DatabasePool;
use crate::data::model::post::dto::GetPostDto;
use crate::data::model::Post;
use crate::data::Result;
use sqlx::query_as;

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
