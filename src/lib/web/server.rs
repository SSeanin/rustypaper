use crate::{data::database::AppDatabase, domain::TokenGenerator, web::router};
use axum::{extract::FromRef, Router};
use axum_extra::extract::cookie::Key;
use http::Method;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub struct Config {
    pub database: AppDatabase,
    pub token_generator: TokenGenerator,
    pub cookie_key: Key,
    pub api_version: &'static str,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub key: Key,
    pub database: AppDatabase,
    pub token_generator: TokenGenerator,
}

impl From<AppState> for Key {
    fn from(value: AppState) -> Self {
        value.key
    }
}

pub async fn generate_app(config: Config) -> (Router, TcpListener) {
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        database: config.database,
        token_generator: config.token_generator,
        key: config.cookie_key,
    };

    let api_routes = Router::new()
        .nest("/posts", router::post::routes(app_state.clone()))
        .nest("/auth", router::auth::routes(app_state));

    let routes = Router::new()
        .nest(format!("/api/v{}", config.api_version).as_str(), api_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::PATCH])
                        .allow_origin(Any),
                )
                .layer(TimeoutLayer::new(Duration::new(60, 0))),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    (routes, listener)
}
