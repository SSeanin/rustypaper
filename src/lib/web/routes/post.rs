use crate::data::database::AppDatabase;
use crate::domain::Post;
use crate::service::action::post::get_post_action;
use crate::web::Result;
use rocket::serde::json::Json;
use rocket::{routes, Route, State};

#[rocket::get("/<shortcode>")]
async fn get_post(shortcode: &str, database: &State<AppDatabase>) -> Result<Json<Post>> {
    let post = get_post_action(shortcode.parse()?, database.get_pool()).await?;
    Ok(Json(post))
}

pub fn routes() -> Vec<Route> {
    routes!(get_post)
}
