use crate::data::database::AppDatabase;
use crate::domain::Post;
use crate::service::action::post::{get_all_posts_action, get_post_action};
use crate::service::object::post::GetPostObject;
use crate::web::form::pagination::PaginationForm;
use crate::web::response::SuccessResponse;
use crate::web::Result;
use rocket::serde::json::Json;
use rocket::{routes, Route, State};

#[rocket::get("/?<pagination>")]
async fn get_all_posts(
    pagination: PaginationForm,
    database: &State<AppDatabase>,
) -> Result<Json<SuccessResponse<Vec<Post>>>> {
    let posts = get_all_posts_action(pagination, database.get_pool()).await?;
    Ok(Json(SuccessResponse::new(posts)))
}

#[rocket::get("/<shortcode>")]
async fn get_post(
    shortcode: &str,
    database: &State<AppDatabase>,
) -> Result<Json<SuccessResponse<Post>>> {
    let post = get_post_action(shortcode.parse::<GetPostObject>()?, database.get_pool()).await?;
    Ok(Json(SuccessResponse::new(post)))
}

pub fn routes() -> Vec<Route> {
    routes!(get_all_posts, get_post)
}
