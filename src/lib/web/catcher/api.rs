use crate::web::response::FailResponse;
use rocket::serde::json::Json;
use rocket::{catch, catchers, Catcher, Request};

#[catch(404)]
fn not_found(req: &Request) -> Json<FailResponse<String>> {
    Json(FailResponse::new(format!(
        "requested resource at {} could not be found on this server",
        req.uri()
    )))
}

pub fn catchers() -> Vec<Catcher> {
    catchers!(not_found)
}
