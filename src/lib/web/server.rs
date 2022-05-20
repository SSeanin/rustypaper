use crate::data::database::AppDatabase;
use crate::web::routes::post;
use rocket::{Build, Rocket};

pub struct RocketConfig {
    pub database: AppDatabase,
}

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    Rocket::build()
        .manage::<AppDatabase>(config.database)
        .mount("/api/v1/posts", post::routes())
}
