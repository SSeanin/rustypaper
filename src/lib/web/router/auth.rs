use crate::data::database::AppDatabase;
use crate::domain::token_generator::{AccessTokenClaims, RefreshTokenClaims};
use crate::domain::{TokenGenerator, User};
use crate::service::action::auth::{login_action, refresh_action};
use crate::service::action::user::create_user_action;
use crate::service::object::user::CreateUserObject;
use crate::web::form::auth::{LoginForm, LoginResponse, RefreshForm, SignupForm};
use crate::web::response::SuccessResponse;
use crate::web::Result;
use rocket::http::{Cookie, CookieJar};
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

#[rocket::post("/login", format = "json", data = "<form>")]
pub async fn login(
    form: Json<LoginForm>,
    database: &State<AppDatabase>,
    token_generator: &State<TokenGenerator>,
    cookies: &CookieJar<'_>,
) -> Result<Json<SuccessResponse<LoginResponse>>> {
    let tokens = login_action(form.into_inner(), database.get_pool(), token_generator).await?;

    let cookie = Cookie::build("Authorization", tokens.access_token)
        .http_only(true)
        // todo prod: set secure
        // .secure(true)
        .finish();

    cookies.add_private(cookie);

    Ok(Json(SuccessResponse::new(LoginResponse {
        refresh_token: tokens.refresh_token,
    })))
}

#[rocket::post("/refresh")]
pub async fn refresh(
    access_token_claims: AccessTokenClaims,
    refresh_token_claims: RefreshTokenClaims,
    database: &State<AppDatabase>,
    token_generator: &State<TokenGenerator>,
    cookies: &CookieJar<'_>,
) -> Result<Json<SuccessResponse<LoginResponse>>> {
    let tokens = refresh_action(
        RefreshForm {
            access_token_claims,
            refresh_token_claims,
        },
        database.get_pool(),
        token_generator,
    )
    .await?;

    let cookie = Cookie::build("Authorization", tokens.access_token)
        .http_only(true)
        // todo prod: set secure
        // .secure(true)
        .finish();

    cookies.add_private(cookie);

    Ok(Json(SuccessResponse::new(LoginResponse {
        refresh_token: tokens.refresh_token,
    })))
}

pub fn routes() -> Vec<Route> {
    routes!(signup, login, refresh)
}
