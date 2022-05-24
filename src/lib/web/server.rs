use crate::data::database::AppDatabase;
use crate::web::catcher::api;
use crate::web::router::post;
use rocket::{Build, Rocket};

pub struct RocketConfig {
    pub database: AppDatabase,
    pub api_version: &'static str,
}

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    Rocket::build()
        .manage::<AppDatabase>(config.database)
        .mount(
            format!("/api/v{}/posts", config.api_version),
            post::routes(),
        )
        .register(format!("/api/v{}", config.api_version), api::catchers())
}
