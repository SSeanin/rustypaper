use std::fmt::Debug;

use axum_extra::extract::cookie::Key;
use dotenvy::dotenv;
use rustypaper::{
    data::database::AppDatabase,
    domain::TokenGenerator,
    web::{server::generate_app, Config},
};
use serde::Deserialize;

const API_VERSION: &str = "0";

#[derive(Debug, Deserialize)]
struct Configuration {
    database_user: String,
    database_host: String,
    database_port: String,
    database_password: String,
    database_name: String,
    auth_shared_secret_key: String,
    cookie_key: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let env = envy::prefixed("RUSTYPAPER_")
        .from_env::<Configuration>()
        .expect("to read configuration environment variables");

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

    let token_generator = TokenGenerator::new(env.auth_shared_secret_key);

    let cookie_key = Key::try_from(env.cookie_key.as_bytes())
        .expect("cookie key to be at lease 64 bytes");

    let (routes, listener) = generate_app(Config {
        database,
        token_generator,
        cookie_key,
        api_version: API_VERSION,
    })
    .await;

    println!(
        "Up and running on {:?}",
        listener.local_addr().expect("to have a running address")
    );

    axum::serve(listener, routes)
        .await
        .expect("to have started");
}
