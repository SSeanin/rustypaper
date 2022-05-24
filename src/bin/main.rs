use dotenv::dotenv;
use rustypaper::data::database::AppDatabase;
use rustypaper::web::{rocket, RocketConfig};
use serde::Deserialize;

const API_VERSION: &str = "0";

#[derive(Debug, Deserialize)]
struct Configuration {
    database_user: String,
    database_host: String,
    database_port: String,
    database_password: String,
    database_name: String,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let env = envy::from_env::<Configuration>()
        .expect("failed to read configuration environment variables");

    let database = AppDatabase::new(
        format!(
            "postgres://{}:{}@{}:{}/{}",
            env.database_user,
            env.database_password,
            env.database_host,
            env.database_port,
            env.database_name
        )
        .as_str(),
    )
    .await;

    let _rocket = rocket(RocketConfig {
        database,
        api_version: API_VERSION,
    })
    .ignite()
    .await?
    .launch()
    .await?;

    Ok(())
}
