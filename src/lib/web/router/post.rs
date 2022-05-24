use crate::data::database::AppDatabase;
use crate::domain::Post;
use crate::service::action::post::get_post_action;
use crate::web::response::SuccessResponse;
use crate::web::Result;
use rocket::serde::json::Json;
use rocket::{routes, Route, State};

#[rocket::get("/<shortcode>")]
async fn get_post(
    shortcode: &str,
    database: &State<AppDatabase>,
) -> Result<Json<SuccessResponse<Post>>> {
    let post = get_post_action(shortcode.parse()?, database.get_pool()).await?;
    Ok(Json(SuccessResponse::new(post)))
}

pub fn routes() -> Vec<Route> {
    routes!(get_post)
}
