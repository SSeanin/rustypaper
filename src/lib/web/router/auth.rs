use crate::data::database::AppDatabase;
use crate::domain::User;
use crate::service::action::user::create_user_action;
use crate::service::object::user::CreateUserObject;
use crate::web::form::auth::SignupForm;
use crate::web::response::SuccessResponse;
use crate::web::Result;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route, State};

#[rocket::post("/signup", format = "json", data = "<form>")]
pub async fn signup(
    form: Json<SignupForm>,
    database: &State<AppDatabase>,
) -> Result<status::Created<Json<SuccessResponse<User>>>> {
    let object: CreateUserObject = form.into_inner().try_into()?;
    let user = create_user_action(object, database.get_pool()).await?;
    Ok(status::Created::new("/me").body(Json(SuccessResponse::new(user))))
}

pub fn routes() -> Vec<Route> {
    routes!(signup)
}
