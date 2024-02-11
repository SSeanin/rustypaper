use crate::{
    domain::{
        token_generator::{AccessTokenClaims, RefreshTokenClaims},
        User,
    },
    service::{
        action::{
            auth::{login_action, refresh_action},
            user::create_user_action,
        },
        object::user::CreateUserObject,
    },
    web::{
        form::auth::{LoginForm, LoginResponse, RefreshForm, SignupForm},
        response::SuccessResponse,
        server::AppState,
        Result,
    },
};
use axum::{
    extract::State,
    http::{header, status::StatusCode, HeaderName},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};

pub async fn signup(
    State(state): State<AppState>,
    Json(form): Json<SignupForm>,
) -> Result<(
    StatusCode,
    [(HeaderName, &'static str); 1],
    Json<SuccessResponse<User>>,
)> {
    let object: CreateUserObject = form.try_into()?;
    let user = create_user_action(object, state.database.get_pool()).await?;
    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, "/me")],
        Json(SuccessResponse::new(user)),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(form): Json<LoginForm>,
) -> Result<(
    StatusCode,
    PrivateCookieJar,
    Json<SuccessResponse<LoginResponse>>,
)> {
    let tokens = login_action(form, state.database.get_pool(), &state.token_generator).await?;

    let cookie = Cookie::build(("Authorization", format!("Bearer {}", tokens.access_token)))
        .http_only(true)
        .path("/api");
    // TODO set secure
    // .secure(true);

    let updated_jar = jar.add(cookie);

    Ok((
        StatusCode::OK,
        updated_jar,
        Json(SuccessResponse::new(LoginResponse {
            refresh_token: tokens.refresh_token,
        })),
    ))
}

pub async fn refresh(
    access_token_claims: AccessTokenClaims,
    refresh_token_claims: RefreshTokenClaims,
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<(
    StatusCode,
    PrivateCookieJar,
    Json<SuccessResponse<LoginResponse>>,
)> {
    let tokens = refresh_action(
        RefreshForm {
            access_token_claims,
            refresh_token_claims,
        },
        state.database.get_pool(),
        &state.token_generator,
    )
    .await?;

    let cookie = Cookie::build(("Authorization", format!("Bearer {}", tokens.access_token)))
        .http_only(true)
        .path("/api");
    // TODO set secure
    // .secure(true);

    let updated_jar = jar.add(cookie);

    Ok((
        StatusCode::OK,
        updated_jar,
        Json(SuccessResponse::new(LoginResponse {
            refresh_token: tokens.refresh_token,
        })),
    ))
}

pub async fn me(
    user: User,
    State(_): State<AppState>,
) -> Result<(StatusCode, Json<SuccessResponse<User>>)> {
    Ok((StatusCode::OK, Json(SuccessResponse::new(user))))
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
        .with_state(state)
}
