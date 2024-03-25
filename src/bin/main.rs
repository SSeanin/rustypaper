use std::fmt::Debug;
use std::fs;

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
    database_user: Option<String>,
    database_host: String,
    database_port: String,
    database_password: Option<String>,
    database_name: String,
    auth_shared_secret_key: Option<String>,
    cookie_key: Option<String>,

    database_user_file: Option<String>,
    database_password_file: Option<String>,
    auth_shared_secret_key_file: Option<String>,
    cookie_key_file: Option<String>,
}

fn read_secret_file_in(path: String) -> String {
    fs::read_to_string(path).expect("secret file to be read in")
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
            if let Some(path) = env.database_user_file {
                read_secret_file_in(path)
            } else {
                env.database_user.expect("at lease one of RUSTYPAPER_DATABASE_USER_FILE or RUSTYPAPER_DATABASE_USER envs to be available")
            },
            if let Some(path) = env.database_password_file {
                read_secret_file_in(path)
            } else {
                env.database_password.expect("at lease one of RUSTYPAPER_DATABASE_PASSWORD_FILE or RUSTYPAPER_DATABASE_PASSWORD envs to be available")
            },
            env.database_host,
            env.database_port,
            env.database_name
        )
        .as_str(),
    )
    .await;

    let token_generator = TokenGenerator::new(
        if let Some(path) = env.auth_shared_secret_key_file {
            read_secret_file_in(path)
        } else {
            env.auth_shared_secret_key.expect("RUSTYPAPER_AUTH_SHARED_SECRET_KEY_FILE or RUSTYPAPER_AUTH_SHARED_SECRET_KEY to be available")
        },
    );

    let cookie_key = Key::try_from(
        (if let Some(path) = env.cookie_key_file {
            read_secret_file_in(path)
        } else {
            env.cookie_key
                .expect("RUSTYPAPER_COOKIE_KEY_FILE or RUSTYPAPER_COOKIE_KEY to be available")
        })
        .as_bytes(),
    )
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
