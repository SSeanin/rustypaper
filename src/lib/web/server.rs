use crate::data::database::AppDatabase;
use crate::web::routes::post;
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
}
