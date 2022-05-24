use crate::data::database::AppDatabase;
use crate::web::routes::post;
use rocket::{Build, Rocket};

pub struct RocketConfig {
    pub database: AppDatabase,
}

pub fn rocket(config: RocketConfig, api_version: &str) -> Rocket<Build> {
    Rocket::build()
        .manage::<AppDatabase>(config.database)
        .mount(format!("/api/v{}/posts", api_version), post::routes())
}
