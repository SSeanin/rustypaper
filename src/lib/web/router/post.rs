use crate::data::database::AppDatabase;
use crate::domain::post::field::Shortcode;
use crate::domain::{Post, User};
use crate::service::action::post::{
    create_post_action, delete_post_action, get_all_posts_action, get_post_action,
    update_post_action,
};
use crate::service::object::post::{CreatePostObject, UpdatePostObject};
use crate::web::form::pagination::PaginationForm;
use crate::web::form::post::{CreatePostForm, UpdatePostForm};
use crate::web::response::SuccessResponse;
use crate::web::Result;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, uri, Route, State};

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
    shortcode: Shortcode,
    database: &State<AppDatabase>,
) -> Result<Json<SuccessResponse<Post>>> {
    let post = get_post_action(shortcode.into_inner().as_str(), database.get_pool()).await?;
    Ok(Json(SuccessResponse::new(post)))
}

#[rocket::post("/", format = "json", data = "<form>")]
async fn create_post(
    user: User,
    form: Json<CreatePostForm>,
    database: &State<AppDatabase>,
) -> Result<status::Created<Json<SuccessResponse<Post>>>> {
    let form = CreatePostForm {
        author_id: user.user_id,
        ..form.into_inner()
    };
    let object: CreatePostObject = form.try_into()?;
    let post = create_post_action(object, database.get_pool()).await?;
    Ok(
        status::Created::new(uri!(get_post(&post.shortcode)).to_string())
            .body(Json(SuccessResponse::new(post))),
    )
}

#[rocket::patch("/<shortcode>", format = "json", data = "<form>")]
async fn update_post(
    shortcode: Shortcode,
    user: User,
    form: Json<UpdatePostForm>,
    database: &State<AppDatabase>,
) -> Result<Json<SuccessResponse<Post>>> {
    let form = UpdatePostForm {
        shortcode: Some(shortcode.into_inner()),
        ..form.into_inner()
    };
    let object: UpdatePostObject = form.try_into()?;
    let post = update_post_action(object, user, database.get_pool()).await?;
    Ok(Json(SuccessResponse::new(post)))
}

#[rocket::delete("/<shortcode>")]
async fn delete_post(
    shortcode: Shortcode,
    user: User,
    database: &State<AppDatabase>,
) -> Result<status::NoContent> {
    let _ = delete_post_action(shortcode.as_str(), user, database.get_pool()).await?;
    Ok(status::NoContent)
}

pub fn routes() -> Vec<Route> {
    routes!(
        get_all_posts,
        get_post,
        create_post,
        update_post,
        delete_post
    )
}
