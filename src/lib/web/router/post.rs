use crate::data::database::AppDatabase;
use crate::domain::Post;
use crate::service::action::post::{
    create_post_action, get_all_posts_action, get_post_action, update_post_action,
};
use crate::service::object::post::{CreatePostObject, GetPostObject, UpdatePostObject};
use crate::web::form::pagination::PaginationForm;
use crate::web::form::post::{CreatePostForm, UpdatePostForm};
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

#[rocket::post("/", format = "json", data = "<form>")]
async fn create_post(
    form: Json<CreatePostForm>,
    database: &State<AppDatabase>,
) -> Result<Json<SuccessResponse<Post>>> {
    let object: CreatePostObject = form.into_inner().try_into()?;
    let post = create_post_action(object, database.get_pool()).await?;
    Ok(Json(SuccessResponse::new(post)))
}

// todo maybe get shortcode as url param
#[rocket::patch("/", format = "json", data = "<form>")]
async fn update_post(
    form: Json<UpdatePostForm>,
    database: &State<AppDatabase>,
) -> Result<Json<SuccessResponse<Post>>> {
    let object: UpdatePostObject = form.into_inner().try_into()?;
    let post = update_post_action(object, database.get_pool()).await?;
    Ok(Json(SuccessResponse::new(post)))
}

pub fn routes() -> Vec<Route> {
    routes!(get_all_posts, get_post, create_post, update_post)
}
