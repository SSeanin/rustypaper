use sqlx::{postgres::PgPoolOptions, Postgres};

pub type AppDatabase = Database<Postgres>;
pub type DatabasePool = sqlx::postgres::PgPool;

pub struct Database<T>(sqlx::Pool<T>)
where
    T: sqlx::Database;

impl Database<Postgres> {
    pub async fn new(database_uri: &str) -> Self {
        let pool = PgPoolOptions::new().connect(database_uri).await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{:?}\n", e);
                eprintln!("Make sure the database connection string is valid. Received database connection string:\n\t {}\n",
                          database_uri);
                panic!("Couldn't connect to the database");
            }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

impl Clone for Database<Postgres> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
