use crate::data::database::AppDatabase;
use crate::domain::TokenGenerator;
use crate::web::catcher::api;
use crate::web::router::auth;
use crate::web::router::post;
use rocket::{Build, Rocket};

pub struct RocketConfig {
    pub database: AppDatabase,
    pub token_generator: TokenGenerator,
    pub api_version: &'static str,
}

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    Rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<TokenGenerator>(config.token_generator)
        .mount(
            format!("/api/v{}/posts", config.api_version),
            post::routes(),
        )
        .mount(format!("/api/v{}/auth", config.api_version), auth::routes())
        .register(format!("/api/v{}", config.api_version), api::catchers())
}
