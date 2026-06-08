use sqlx::{SqlitePool, sqlite::SqliteConnectOptions, sqlite::SqlitePoolOptions};
use std::str::FromStr;

pub async fn create_pool() -> SqlitePool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_lazy(&url)
        .expect("Failed to create pool")
}
