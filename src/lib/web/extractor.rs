use crate::{
    domain::{
        token_generator::{AccessTokenClaims, RefreshTokenClaims},
        User,
    },
    service::{action::user::get_user_by_id_action, ServiceError},
    web::server::AppState,
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};

fn validate_authorization_cookie(cookie: Cookie<'_>) -> std::result::Result<String, ServiceError> {
    let token_parts = cookie
        .value_trimmed()
        .split_whitespace()
        .collect::<Vec<&str>>();

    if token_parts[0] == "Bearer" {
        Ok(String::from(token_parts[1]))
    } else {
        Err(ServiceError::InvalidToken)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AccessTokenClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ServiceError);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let private_cookie_jar: PrivateCookieJar<AppState> =
            PrivateCookieJar::from_request_parts(parts, state)
                .await
                .map_err(|err| match err {})?;

        let cookie = private_cookie_jar.get("Authorization");

        let access_token = if let Some(cookie) = cookie {
            match validate_authorization_cookie(cookie) {
                Ok(token) => token,
                Err(_) => return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken)),
            }
        } else {
            return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken));
        };

        let state = AppState::from_ref(state);

        match state
            .token_generator
            .verify_token::<AccessTokenClaims>(access_token.as_str())
        {
            Err(_) => Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken)),
            Ok(claims) => Ok(claims),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RefreshTokenClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ServiceError);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let refresh_token = if let Some(token_header) = parts.headers.get("refresh-token") {
            let token = token_header
                .to_str()
                .unwrap_or_default()
                .split_whitespace()
                .collect::<Vec<&str>>();

            if token[0] == "Bearer" {
                token[1]
            } else {
                return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken));
            }
        } else {
            return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken));
        };

        let state = AppState::from_ref(state);

        match state
            .token_generator
            .verify_token::<RefreshTokenClaims>(refresh_token)
        {
            Err(_) => Err((StatusCode::UNAUTHORIZED, ServiceError::Unauthorized)),
            Ok(claims) => Ok(claims),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ServiceError);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let private_cookie_jar: PrivateCookieJar<AppState> =
            PrivateCookieJar::from_request_parts(parts, state)
                .await
                .map_err(|err| match err {})?;

        let cookie = private_cookie_jar.get("Authorization");

        let access_token = if let Some(cookie) = cookie {
            match validate_authorization_cookie(cookie.clone()) {
                Ok(token) => token,
                Err(_) => return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken)),
            }
        } else {
            return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken));
        };

        let state = AppState::from_ref(state);

        let claims = match state
            .token_generator
            .verify_token::<AccessTokenClaims>(access_token.as_str())
        {
            Ok(claims) => claims,
            Err(_) => return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken)),
        };

        match get_user_by_id_action(claims.sub, state.database.get_pool()).await {
            Ok(user) => Ok(user),
            Err(_) => return Err((StatusCode::UNAUTHORIZED, ServiceError::InvalidToken)),
        }
    }
}
