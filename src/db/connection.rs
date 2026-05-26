use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn create_pool() -> SqlitePool {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to SQLite")
}
